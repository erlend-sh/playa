<div align="center">
  <span style="font-size: 64px;">🏖️</span>
  <h1 align="center">playa</h1>
  <p align="center">
    Decentralized Social Platform powered by Rust and Whizzes Contributors
  </p>
</div>

<div align="center">

  [![Discord](https://img.shields.io/discord/1011702194925490186?color=blue&label=discord&logo=discord)](https://discord.gg/yde6mcgs2C)
  ![Build](https://github.com/whizzes/playa/workflows/build/badge.svg)
  ![Clippy](https://github.com/whizzes/playa/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/whizzes/playa/workflows/fmt/badge.svg)

</div>

## Development

### Requirements

- [Rust](https://rustup.rs)
- [Docker](https://www.docker.com)
- [Justfile](https://github.com/casey/just) (**Recommended**)

### Getting Started

```bash
# install just command runner
cargo install just

# clone this repository
git clone https://github.com/whizzes/playa.git


# step into repository directory
cd ./playa

# open a termital window and spin up Docker Containers
just dev

# create a new terminal window and run database migrations
just prepare

# execute the server (next time you just run this command)
cargo run serve
```

> Note: As of today migrations runs when bootstrapping the server automatically

## Software Architecture

### Layers

Client traffic is handled by a GraphQL endpoint, domain logic is exposed through services
which encapsulate access to the database logic. The CLI solution communicates directly to
the services.

<div align="center">

  ![softarq](https://github.com/whizzes/playa/assets/34756077/86abfb8d-8e96-4e93-9677-4e0864f53da6)

</div>

### Design

This project takes some inspiration on Domain Driven Design, but does **not** implements
its concepts completely. Concepts like value object, model, repositories and services are
present but are not 100% accurate to the original Domain Driven Design architecture.

### Modules

The client and server solution is available in this repository.

Directory | Description
--- | ---
`client` | Web Front-End, written in SvelteKit & TypeScript
`crates/` | Contains GraphQL Server Logic, CLI and Domain libraries. Rust is the predominant language.
`crates/cli` | CLI used to manage the Server instance. run database migrations and other developer tasks
`crates/core` | Domain Logic, includes Models, Value Objects, Repositories and Services
`crates/server` | HTTP Server Logic, uses Axum and GraphQL
`crates/entities` | Entities generated from database
`crates/migrations` | Database migrations

## Testing

## Running E2E Tests

E2E Tests run by default on a different database, this allow us to have a
database for development and another for testing, speeding up the development
process.

The dedicated E2E Database will be builded along with other containers when running `just dev`.

You can also execute a single E2E Test by specifying the name of the test
function along with the `just e2e_test` command:

```bash
just e2e_test my_super_test_function_name
```

To run every test just execute:

```bash
just e2e_test
```

Teardown containers using `just undev`.


## Contributors

<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%">
        <a href="https://github.com/EstebanBorai">
          <img src="https://avatars.githubusercontent.com/u/34756077?v=4?s=100" width="100px;" alt="Esteban Borai"/>
          <br />
          <sub>
            <b>Esteban Borai</b>
          </sub>
        </a>
        <br />
        <a href="https://github.com/whizzes/playa/commits?author=EstebanBorai" title="Code">💻</a>
      </td>
      <td align="center" valign="top" width="14.28%">
        <a href="https://github.com/Michael-Liendo">
          <img src="https://avatars.githubusercontent.com/u/70660410?v=4?s=100" width="100px;" alt="Michael Liendo"/>
          <br />
          <sub>
            <b>Michael Liendo</b>
          </sub>
        </a>
        <br />
        <a href="https://github.com/whizzes/playa/commits?author=Michael-Liendo" title="Code">💻</a>
      </td>
      <td align="center" valign="top" width="14.28%">
        <a href="https://github.com/Dave136">
          <img src="https://avatars.githubusercontent.com/u/49698182?v=4?s=100" width="100px;" alt="David Arenas"/>
          <br />
          <sub>
            <b>David Arenas</b>
          </sub>
        </a>
        <br />
        <a href="https://github.com/whizzes/playa/commits?author=Dave136" title="Code">💻</a>
      </td>
      <td align="center" valign="top" width="14.28%">
        <a href="https://github.com/CudiLala">
          <img src="https://avatars.githubusercontent.com/u/88282186?s=64&v=4?s=100" width="100px;" alt="CudiLala"/>
          <br />
          <sub>
            <b>Cudi Lala</b>
          </sub>
        </a>
        <br />
        <a href="https://github.com/whizzes/playa/commits?author=CudiLala" title="Code">💻</a>
      </td>
      <td align="center" valign="top" width="14.28%">
        <a href="https://github.com/PalyZambrano">
          <img src="https://avatars.githubusercontent.com/u/29868332?v=4?s=100" width="100px;" alt="Paly Zambrano"/>
          <br />
          <sub>
            <b>Paly Zambrano</b>
          </sub>
        </a>
        <br />
        <a href="https://github.com/whizzes/playa/commits?author=PalyZambrano" title="Code">💻</a>
      </td>
      <td align="center" valign="top" width="14.28%">
        <a href="https://github.com/Phosphorus-M">
          <img src="https://avatars.githubusercontent.com/u/19656993?v=4?s=100" width="100px;" alt="Phosphorus-M"/>
          <br />
          <sub>
            <b>Phosphorus-M</b>
          </sub>
        </a>
        <br />
        <a href="https://github.com/whizzes/playa/commits?author=Phosphorus-M" title="Code">💻</a>
      </td>
      <td align="center" valign="top" width="14.28%">
        <a href="https://github.com/dararod">
          <img src="https://avatars.githubusercontent.com/u/29829194?v=4?s=100" width="100px;" alt="David Rodriguez"/>
          <br />
          <sub>
            <b>David Rodríguez</b>
          </sub>
        </a>
        <br />
        <a href="https://github.com/whizzes/playa/commits?author=dararod" title="Code">💻</a>
      </td>
    </tr>
  </tbody>
</table>

## License

Licensed under the MIT License
