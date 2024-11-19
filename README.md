# FiveM Resource Manager

[**NPM Package Link**](https://www.npmjs.com/package/frs-manager)

This is a Command Line Interface made to make it easier for you to create a new FiveM resource.<br>
You can pick between any of the popular FiveM/UI frameworks out there and the CLI will set up a boilerplate resource for you.<br>

# Installation

Run `npm install -g frs-manager` in a console/terminal.

# Usage

To create a new resource:

- Run `frs new` in any console/terminal
- If you chose to use an actual ui **framework**, such as Vue/React, run `npm install` in the `ui` folder of the project after creating a new project
- Then you will be able to use Vue/React as normal. Building would be done by `npm run build`, and so on

## Requirements

- [Node.js](https://nodejs.org/en/)
- A FiveM server if you actually want to use the files generated from this CLI.

# FiveM Boilerplate types

- Standalone / No framework
- [QB-Core](https://github.com/qbcore-framework)
- [ESX](https://github.com/esx-framework/esx-legacy)

# UI Boilerplate types

- [Vue 3.0 (JS & TS)](https://vuejs.org/)
- [React (JS & TS)](https://reactjs.org/)
- [JQuery (JS)](https://jquery.com/)
- No framework / Standard JS
- No UI
