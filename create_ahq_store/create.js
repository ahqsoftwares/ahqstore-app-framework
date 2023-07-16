#!/usr/bin/env node
async function esRequire(package, def = true) {
    const pkg = await import(package);
    return def ? pkg.default : pkg;
}

(async() => {
    /**
     * @type {import("chalk").default}
     */
    const chalk = await esRequire("chalk");
    /**
     * @type {import("inquirer").default}
     */
    const inquirer = await esRequire("inquirer");

    console.log(chalk.green("-----------------------------------------"));
    console.log(chalk.green("       AHQ Store App Creation Tool       "));
    console.log(chalk.green("-----------------------------------------"));

    inquirer.prompt(
        [
            {
                name: chalk.yellow("What is your app name?"),
                filter: (/** @type {string} */ r) => r.replace(/ /g, "-").substring(0, 32),
                validate: (/** @type {string} */ r) => r.length > 3 ? true : chalk.redBright("At least 3 characters")
            },
            {
                name: chalk.yellow("What is your app description?"),
                filter: (/** @type {string} */ r) => r.substring(0, 256),
                validate: (/** @type {string} */ r) => r.length > 3 ? true : chalk.redBright("At least 3 characters")
            },
            {
                name: chalk.yellow("Who is the author?"),
                filter: (/** @type {string} */ r) => r.split(","),
                validate: (/** @type {string[]} */ r) => r.filter((r) => r.length >= 3).length > 0 ? true : chalk.redBright("At least 3 characters")
            },
            {
                name: chalk.yellow("What is your app id?"),
                filter: (/** @type {string} */ r) => r.replace(/ /g, "-").substring(0, 20),
                validate: (/** @type {string} */ r) => r.length > 16 ? true : chalk.redBright("At least 16 characters")
            }
        ]
    )
})()