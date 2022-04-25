# Rust static site generator

This project is a very simple static site generator written in Rust.

## How it works

Let's say that we have the following project structure:

```
.
└── src
    ├── _include
    │   ├── footer.html
    │   └── header.html
    └── index.html
```

All the files are in a `src` folder and the special `_include` folder is for html files composition.

By using the syntax `{% file.html %}`, you can include html files in others. This allows to reuse html elements on multiple pages.

To build your website, run the `build` action in your project's directory:

```bash
$ static-site-rs build
```

With the above example it will do the following:

```
[INFO] Starting build...
[INFO] Using src folder at "/websites/test-static/src"
[INFO] Build folder doesn't exist, creating it...
[INFO] Using force mode: false
[INFO] Copying file index.html to build/index.html
[INFO] Found include directive header.html -> index.html
[INFO] Found include directive footer.html -> index.html
[INFO] Build successfull!
```

`index.html` had this content:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Home page</title>
  </head>
  <body>
    {% header.html %}

    <main>The main content</main>

    {% footer.html %}
  </body>
</html>
```

Now the build version includes the header and footer:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Home page</title>
  </head>
  <body>
    <header>
      <h1>Header</h1>
    </header>

    <main>The main content</main>

    <footer>
      <p>Footer...</p>
    </footer>
  </body>
</html>
```

## TODO

- Handle UTF-8 characters spanning across multiple bytes (since replacing content in a file is shifted...)

- Use caching when building the files again

- Handle more sophisticated syntax for directives `{% %}` like variables and keywords. For example it could be `{% include test.html %}` and `{% var title %}`

- Register processor for handling different file types like css, html, js or images...
