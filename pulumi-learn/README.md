# Pulumi Learning Notes

A walkthrough of everything we did to get a Pulumi TypeScript project running locally from scratch.

---

## 1. Prerequisites

Three tools need to be on your PATH before anything works.

| Tool | What it does | Check |
|------|-------------|-------|
| Pulumi CLI | Orchestrates deployments | `pulumi version` |
| Node.js 18+ | Runs TypeScript programs | `node --version` |
| npm | Installs packages | `npm --version` |

---

## 2. Local state backend

By default Pulumi wants to store state in the cloud (app.pulumi.com). For local learning you don't need that.

```bash
pulumi login --local
pulumi whoami   # should print your username, not a cloud URL
```

State is now stored in `~/.pulumi/stacks/` as plain JSON files — you can read them directly.

### Remove the passphrase prompt

Pulumi encrypts secrets in state with a passphrase. Locally there's nothing sensitive, so skip it permanently:

```bash
echo 'export PULUMI_CONFIG_PASSPHRASE=""' >> ~/.zshrc
source ~/.zshrc
```

If you forget to set it you'll get:
```
error: getting stack configuration: incorrect passphrase
```
Fix: prefix the command with `PULUMI_CONFIG_PASSPHRASE="" pulumi ...`

---

## 3. Scaffold a new project

```bash
mkdir pulumi-learn && cd pulumi-learn
pulumi new typescript --yes --name pulumi-learn --stack dev
```

This generates:
- `Pulumi.yaml` — project metadata (name, runtime)
- `Pulumi.dev.yaml` — stack-specific config (per environment)
- `index.ts` — your infrastructure program
- `package.json` — npm dependencies

---

## 4. Add a provider

Providers are npm packages that give you resource types to work with.
We used `@pulumi/random` to create random strings without needing any cloud credentials.

```bash
npm install @pulumi/random
```

Other common providers: `@pulumi/aws`, `@pulumi/azure-native`, `@pulumi/gcp`, `@pulumi/kubernetes`.

---

## 5. Write your program

Pulumi programs are just TypeScript. You declare resources as objects and Pulumi figures out what to create/update/delete.

```typescript
import * as pulumi from "@pulumi/pulumi";
import * as random from "@pulumi/random";

const resourceA = new random.RandomString("resource-a", {
    length: 8,
    special: false,   // no !@#$ etc.
});

const resourceB = new random.RandomString("resource-b", {
    length: resourceA.result.apply(s => s.length * 2),
    special: true,
});

export const stringA = resourceA.result;
export const stringB = resourceB.result;
```

### Understanding Output<T>

This is the most important concept in Pulumi.

`resourceA.result` is not a `string` — it's an `Output<string>`. The actual value doesn't exist yet when your program runs; Pulumi resolves it during deployment.

To use an Output value as input to another resource, use `.apply()`:

```typescript
// Wrong — result is Output<string>, not string
const len = resourceA.result.length;   // TypeScript error

// Right — .apply() unwraps the value when it's available
const len = resourceA.result.apply(s => s.length);
```

When resource B's `length` input depends on resource A's output, Pulumi automatically knows to create A first, then B. This is how you express dependencies — not by calling functions in order, but by wiring outputs to inputs.

### Exports

```typescript
export const stringA = resourceA.result;
```

Exported values show up in `pulumi stack output` after deployment. Useful for seeing what got created, or passing values to other systems.

---

## 6. Deploy

```bash
pulumi up
```

Pulumi shows you a preview (what it's about to do) and asks for confirmation. To skip the prompt:

```bash
pulumi up --yes
```

After deployment:
```bash
pulumi stack output
# OUTPUT   VALUE
# stringA  13IWukQ2
# stringB  ueEL1Hi%92W+>KCE
```

---

## 7. Inspect the state file

After `pulumi up`, Pulumi writes the current state to:

```
~/.pulumi/stacks/<project-name>/dev.json
```

Open it and you'll see each resource with:
- `urn` — unique resource name (stack + project + type + logical name)
- `type` — e.g. `random:index/randomString:RandomString`
- `inputs` — what you passed in (`length: 8`, `special: false`)
- `outputs` — what the provider returned (`result: "13IWukQ2"`)
- `dependencies` — which other resources this one depends on
- `propertyDependencies` — which specific *properties* depend on which resources

The `propertyDependencies` for resource-b looked like this:
```json
"propertyDependencies": {
    "length": ["urn:...::resource-a"]
}
```
Pulumi tracked that `length` specifically came from resource-a's output — not just a general dependency.

---

## 8. Tear down

```bash
pulumi destroy --yes
```

Deletes all resources in the stack. The state file's resource array becomes empty (the stack itself still exists, just with no resources in it).

---

## 9. Bonus example: write a local HTML file

This ties everything together and introduces two new things:
- `@pulumi/command` — a provider for running local shell commands as resources
- `pulumi.interpolate` — the clean way to build strings from `Output<T>` values

### Install the command provider

```bash
npm install @pulumi/command
```

### The program

```typescript
import * as pulumi from "@pulumi/pulumi";
import * as random from "@pulumi/random";
import { local } from "@pulumi/command";

// Generate a random hex color for the page background
const color = new random.RandomString("bg-color", {
    length: 6,
    special: false,
    upper: false,
});

// Generate a random animal name for the greeting
const pet = new random.RandomPet("greeting-pet", {
    length: 2,
});

// pulumi.interpolate is like a template literal, but it handles Output<T> values.
// You cannot use `${}` directly in a normal template string with Output values —
// you'd get "[object Object]". Use pulumi.interpolate instead.
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
// `create` runs on pulumi up, `delete` runs on pulumi destroy.
// `triggers` tells Pulumi to re-run the command when these values change.
const page = new local.Command("hello-page", {
    create: html.apply(h => `cat > hello.html << 'PULUMIEOF'\n${h}\nPULUMIEOF`),
    delete: "rm -f hello.html",
    triggers: [color.result, pet.id],
});

export const background = pulumi.interpolate`#${color.result}`;
export const greeting = pet.id;
```

### What you get

After `pulumi up`, a `hello.html` file appears next to your `index.ts` with a random background color and a random animal greeting. Every `pulumi up` with changed outputs writes a new one. `pulumi destroy` cleans it up.

```bash
pulumi stack output
# OUTPUT      VALUE
# background  #3af2c1
# greeting    honest-iguana

open hello.html   # view it in the browser
```

### `pulumi.interpolate` vs `.apply()`

Both let you work with `Output<T>` values, but they're used differently:

```typescript
// Use interpolate when building a string with multiple outputs mixed in
const message = pulumi.interpolate`Color: #${color.result}, Pet: ${pet.id}`;

// Use .apply() when you need to transform a value or run logic
const upperGreeting = pet.id.apply(name => name.toUpperCase());
const doubleLength = color.result.apply(s => s.length * 2);
```

Think of `pulumi.interpolate` as the Pulumi-aware version of a template literal.

---

## Key concepts summary

| Concept | What it is |
|---------|-----------|
| Stack | One deployment environment (dev, staging, prod) |
| Resource | A thing Pulumi manages (a string, a bucket, a VM) |
| Provider | Plugin that knows how to create a type of resource |
| `Output<T>` | A value that will exist after deployment — wire these between resources to express dependencies |
| `.apply()` | How you transform an `Output<T>` into another `Output<U>` |
| State file | JSON record of everything Pulumi created — source of truth for diffs |
| `pulumi up` | Deploy (create/update) |
| `pulumi destroy` | Tear down everything |
| `pulumi stack output` | Print exported values |
