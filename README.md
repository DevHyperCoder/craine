# CRAINE Rust HTML Compiler with Components

CRAINE is a powerful html compiler allowing for components in pure html without using javascript. Currently in alpha but growing every day. 

- **Fast and effcient:** Only compiles the required components and because it is built with rust, it is blazing fast.
- **Easy to learn:** If you know html, you are good to go with craine as the syntax is just html (future additions maybe added but adhering to html)
- **No JavaScript required:** Unlike frameworks like react.js, svelte etc, craine is a compiler and does not use javascript. (SvelteJS is also a compiler but it does use javascript).
You **can** use javascript but it is **not required** to use craine

## Installation
> Requires the rust compiler and cargo to be in path

- Clone the repo and `cd` into the directory
- Run `cargo check` This compiles the program and informs it there are any errors. If any, please file a issue.
- You can install it globally by `cargo install --path .`

## Tutorial

The tutorial is split between a few parts.

### Pages vs Components
craine uses the concept of pages and components.

- **Page:** A page is a document that is rendered by the browser, containing `<!DOCTYPE>`, `<html> </html>` etc. Pages can have components but pages can not be imported in other components or pages
A page always begins with a capital letter (PascalCase)
- **Components:** A component is like a split html fragment, ie without `<!DOCTYPE>`, `<html> </html>` etc. Components can contain other component and can be imported into pages/components.
A component always begins with a small letter (camelCase)

> NOTE: The filename restrictions are not conventions. The compiler is built to work that way

### Importing other components
CRAINE uses the `import <path>` syntax to import other components. `<path>` can be a relative path or a absolute path. Prefer keeping the imports at the top of the files. A component which is never imported is termed as a "Unused Component"

### First Project

In a empty directory, create a "src" directory and inside that create a page named `index.html`

`index.html`

```html
import ./components/FancyButton.html

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>CRAINE Tutorial</title>
</head>
<body>
    <FancyButton />
</body>
</html>
```

Now, create a components directory inside the source dir and then create the FancyButton.html file

Notice how we used `<FancyButton />` like a tag in html ? that is how craine works in the background.

`FancyButton.html`

```html
<button class="fancy">Click me!!</button>
```

This is the FancyButton component, just containing a html fragment.

### Variables

Variables in CRAINE are very similar to "props" in react.js ; Variables in CRAINE are also scoped variables.

**Syntax:**

To define a variable, `{name||value}` is used. If a new variable is declared, the value is overriden. 

To get the value of a variable, `(name)` is used. A variable's value can be used in classes, id, attributes and text inside the component.

> In the future, this will be changed to use `{name}` to remain consistent with the definition syntax

**Example**

Modify the contents of the body tag:

```html
<body>
    <FancyButton>
        {color||blue}
    </FancyButton>
</body>
```

And, modify the `FancyButton` component with this:

```html
<style>
    .fancy {
        background-color: "(color)"
    }
</style>

<button class="fancy">Click me!!</button>
```

### Compiling with CRAINE

The directory with the html files is termed as the working directory. Assuming craine is installed and the binary is in `$PATH`, run `craine <path to working directory>` in a terminal.

This would spit out the html into the terminal (ie, stdout). The example here shows formatted html so it is easy to follow

```html
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <title>CRAINE Tutorial</title>
</head>
<body>
    <div>
        <style>
            .fancy {
                background-color: "(color)"
            }
        </style>
        <button class="fancy"> I am a fancy button </button>
    </div>
</body>
</html>
```

You would now be able to see that, `<FancyButton />` got replaced with a div enclosing the component file.

### Configuration

CRAINE currently accepts a `.craine` or `craine.json` file for the following parameters:
- `build_dir`: Sets the output directory.
- `src_dir` : Sets the source directory.

Config files are in json format.

### Command line options

#### init

Initializes a empty craine project
- `--path` : Required ; Path to a empty directory to initialize a craine project.

#### compiles

Compiles a craine project
- `--path` : Required ; Path to a craine workspace directory (ie, a directory containing a craine config file)
- `--autorun` : Optional ; Automatically compiles when any file inside the src directory changes

## LICENSE
CRAINE is licensed under [GPL-3.0](./LICENSE)
