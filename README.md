# This repository is deprecated

Due to a lack of time we sadly have decided to deprecate this repository. It will stay public for a while, but no issues will be reviewed or pull requests will be merged. Feel free to fork the repository to continue the work if that is desired.

<p align="center">
  <br />
  <img
    alt="Siera logo"
    src="./images/siera-dark.svg#gh-dark-mode-only"
    height="250px"
  />
   <img
    alt="Siera logo"
    src="./images/siera-light.svg#gh-light-mode-only"
    height="250px"
  />
</p>

<h1 align="center" ><b>Siera CLI</b></h1>

<h3 align="center">Powered by Hyperledger Aries and &nbsp; <img src="./images/animo-logo-dark-background.png#gh-dark-mode-only" height="12px"><img src="./images/animo-logo-light-background.png#gh-light-mode-only" height="12px"></h3><br>

<p align="center">
<a href="#getting-started">Getting started</a> &nbsp;|&nbsp;
  <a href="#roadmap">Roadmap</a> &nbsp;|&nbsp;
  <a href="#contributing">Contributing</a> 
    
</p>

<p align="center">
    <a href="https://crates.io/crates/siera"><img src="https://img.shields.io/crates/v/siera.svg" alt="crates.io"></a>
    <a href="https://github.com/animo/siera-cli/actions/workflows/check.yml"><img src="https://github.com/animo/siera-cli/actions/workflows/check.yml/badge.svg" alt="Check"></a>
    <a href="https://github.com/animo/siera-cli/actions/workflows/test.yml"><img src="https://github.com/animo/siera-cli/actions/workflows/test.yml/badge.svg" alt="Test"></a>
    <a href="https://github.com/animo/siera-cli/actions/workflows/e2e-test.yml"><img src="https://github.com/animo/siera-cli/actions/workflows/e2e-test.yml/badge.svg" alt="E2E tests"></a>
</p>

Looking for more UI and less CLI? Try out the new [Siera Desktop](https://siera.animo.id/desktop) application.

Siera is the most convenient way for self-sovereign identity (SSI) developers to interact with SSI agents.

-   **Environments** to easily manage configuration for multiple projects and agents
-   **Automations** that you can perform against an agent
-   **Mock data** so that you can focus on the important task of building your application instead of other foobar (coming soon 🚧)

If you are looking for more information about the concepts, example code and tutorials on how to use the CLI we recommend you check out our extensive [docs](https://docs.siera.animo.id/).

## Installation

Heres how to install Siera using the most popular package managers. For advanced installation options, binaries and troubleshooting we recommend checking out the [installation guide](https://docs.siera.animo.id/guides/installation).

### macOS using Brew

```sh
brew tap animo/siera
brew install siera
```

### Cargo install

```sh
cargo install --git https://github.com/animo/siera-cli
```

### Manual build from source

```sh
git clone https://github.com/animo/siera-cli
cd siera-cli

# Run ONE of the following commands to build:
make build

cargo build --release

# Run ONE of the following commands to add install the siera into your PATH
make install

cargo install --path .

```

## Getting started

In order to work with Siera an agent needs to be configured. With the following command you connect with our community agent and can access all the functionality.

```sh
siera configuration add --default
```

This command will set up the community agent. To set up your own agent or use our multi-tenant agent to get your own environment, see the [set up guide](https://docs.siera.animo.id/guides/configuration).

If you are getting started with the tool we recommend enabling informational logs by passing the `--verbose` (or `-v`) flag.

To see all actions simply use the `--help` or `-h` flag or see the [feature overview](https://docs.siera.animo.id/features/introduction).

## Roadmap

We intend to support multiple versions of the Aries agent. See the CLI help `siera --help` for a list of actions we currently support.

| Feature                             | Status | Description                                                                     |
| ----------------------------------- | ------ | ------------------------------------------------------------------------------- |
| Environments                        | ✅     | Support multiple environments.                                                  |
| Connections                         | ✅     | Retrieve connections or create invitations.                                     |
| Schemas                             | ✅     | Retrieve or create schemas.                                                     |
| Credentials                         | ✅     | Offer or propose credentials.                                                   |
| ACA-Py 0.7.3 support                | ✅     | ACA-Py 0.7.3 support.                                                           |
| Verbosity                           | ✅     | Three verbosity levels and error messages.                                      |
| Automation: offer credential mocked | ✅     | Offer a mocked data credential through an automation.                           |
| Community agent                     | ✅     | Default use with the Animo community agent.                                     |
| Multitenancy                        | ✅     | Use a personal agent environment with your personal token.                      |
| Brew install                        | ✅     | Brew install.                                                                   |
| Proofs                              | ✅     | Proofs.                                                                         |
| Apt-get install                     | ✅     | Apt-get install.                                                                |
| Chocolaty install                   | ✅     | Chocolaty install.                                                              |
| Automation: offer credential custom | ✅     | Offer a custom data credential through an automation.                           |
| Filters                             | 🚧     | Use filters to determine what output you want returned.                         |
| Automation: definitions custom      | ✅     | Create a schema + credential definition with custom data through an automation. |
| Automation: proofs mock             | 🚧     | Present proof with mock data through an automation.                             |
| Automation: proofs custom           | 🚧     | Present proof with custom data through an automation.                           |

## Contributing

Is there something you'd like to fix or add to the CLI? Great! We 💗 community
contributions. [Get involved](https://docs.siera.animo.id/community/contributing).
