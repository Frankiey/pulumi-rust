import * as pulumi from "@pulumi/pulumi";
import * as random from "@pulumi/random";
import { local } from "@pulumi/command";

// --- Part 1: two random strings with an Output dependency ---

const resourceA = new random.RandomString("resource-a", {
    length: 8,
    special: false,
});

// B's length is derived from A's Output<string> — double A's character count
const resourceB = new random.RandomString("resource-b", {
    length: resourceA.result.apply(s => s.length * 2),
    special: true,
});

export const stringA = resourceA.result;
export const stringB = resourceB.result;

// --- Part 2: write a local HTML file ---

// Random hex color for the page background
const color = new random.RandomString("bg-color", {
    length: 6,
    special: false,
    upper: false,
});

// Random animal name for the greeting
const pet = new random.RandomPet("greeting-pet", {
    length: 2,
});

// pulumi.interpolate is like a template literal, but works with Output<T> values
const html = pulumi.interpolate`<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Hello from Pulumi</title>
  <style>
    body {
      margin: 0;
      height: 100vh;
      display: flex;
      align-items: center;
      justify-content: center;
      background: #${color.result};
      font-family: sans-serif;
    }
    h1 {
      color: white;
      font-size: 3rem;
      text-shadow: 1px 2px 6px rgba(0,0,0,0.5);
    }
  </style>
</head>
<body>
  <h1>Hello, ${pet.id}!</h1>
</body>
</html>`;

// local.Command runs a shell command as a Pulumi resource.
// create = runs on pulumi up, delete = runs on pulumi destroy.
// triggers = re-runs the command whenever these Output values change.
const page = new local.Command("hello-page", {
    create: html.apply(h => `cat > hello.html << 'PULUMIEOF'\n${h}\nPULUMIEOF`),
    delete: "rm -f hello.html",
    triggers: [color.result, pet.id],
});

export const background = pulumi.interpolate`#${color.result}`;
export const greeting = pet.id;
