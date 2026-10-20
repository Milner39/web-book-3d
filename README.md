# Web Book-3D

<p align="center">
  <img src="https://img.shields.io/badge/nix-flake-5277c3?logo=nixos&logoColor=white" />
  <img src="https://img.shields.io/badge/Deno-FFFFFF?style=flat&logo=Deno&logoColor=black" />
  <img src="https://img.shields.io/badge/Rust-D2991D?style=flat&logo=Rust&logoColor=white" />
  <img src="https://img.shields.io/badge/Axum-000000?style=flat&logo=Tokio&logoColor=white" />
  <img src="https://img.shields.io/badge/Tauri-24C8D8?style=flat&logo=Tauri&logoColor=white" />
  <img src="https://img.shields.io/badge/Svelte-FF3E00?style=flat&logo=Svelte&logoColor=white" />
</p>

<hr>

Tauri/Svelte based application/website that uses Three.js and the "HTML-in-Canvas" API to 
render a book with pages that can be customised with a variety of HTML content, 
animation, and more.

## Developing

### Setup

Enter the dev shell, which provides Rust (through `rustup`), Deno and the WebKitGTK
libraries that Tauri needs (x86_64 Linux only):

```sh
nix develop
```

Install the frontend's JavaScript dependencies (again whenever they change):

```sh
(cd src-svelte && deno install)
```

The Rust version is pinned in `rust-toolchain.toml`, and `rustup` installs it on the first
`cargo` command.

### Running

Run everything from the repository root through `deno task`, as the tasks load the dev
settings the servers need.

1. Start the Axum server, which handles the API:

   ```sh
   deno task dev:axum
   ```

   Wait until it prints `listening on http://127.0.0.1:3000`.

2. In a second terminal, start the website, the desktop app, or both:

   | Client      | Command               | Then                                   |
   | ----------- | --------------------- | -------------------------------------- |
   | Website     | `deno task dev:web`   | Open <http://127.0.0.1:5173>           |
   | Desktop app | `deno task dev:tauri` | The app window opens on its own        |

   Both send their requests to the same Axum server, and both reload when you edit the
   Svelte code.

The first `dev:axum` and `dev:tauri` take a few minutes while Cargo compiles their
dependencies. Subsequent starts are faster.
