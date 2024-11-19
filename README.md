# FiveM Resource Manager

[**NPM Package Link**](https://www.npmjs.com/package/frs-manager)

This is a Command Line Interface made to make it easier for you to create and manage FiveM resource.

You can create new template resources with any of hte popula popular FiveM/UI frameworks out there and the CLI will set everything up for you

You can then use the bundling utility to bundle the resource into a keymaster ready zip-file.

# Installation

Run `npm install -g frs-manager` in a console/terminal.

# Previously known as..

This repo is a collection of [Z3rio/fivem-resource-cli](https://github.com/Z3rio/fivem-resource-cli) and [Z3rio/bundler-cli](https://github.com/Z3rio/bundler-cli)

# Usage

## Resource Creator

To create a new resource:

- Run `frs new` in any console/terminal
- If you chose to use an actual ui **framework**, such as Vue/React, run `npm install` in the `ui` folder of the project after creating a new project
- Then you will be able to use Vue/React as normal. Building would be done by `npm run build`, and so on

### FiveM Boilerplate types

- Standalone / No framework
- [QB-Core](https://github.com/qbcore-framework)
- [ESX](https://github.com/esx-framework/esx-legacy)

### UI Boilerplate types

- [Vue 3.0 (JS & TS)](https://vuejs.org/)
- [React (JS & TS)](https://reactjs.org/)
- [JQuery (JS)](https://jquery.com/)
- No framework / Standard JS
- No UI

## Resource Bundler

... to be written

# Requirements

- [Node.js](https://nodejs.org/en/)
