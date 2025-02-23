# code-templates

Code tamplates CLI program. Allows `template` parsing over a single input variable `name` into a desider `directory`.

## Instalation

1. Clone the repository
2. Run `$ cargo biuld --release`

The desired `executable` will be located inside the `target` folder.

## Configuration

This program is configured with `env` variables.
1. `BLUEPRINTS_PATHS` will store the places to look for the code templates, the paths must be separated by semicolons. Example: `"BLUEPRINTS_PATHS" = "./.blueprints; ~/Another/Route"`.
2. By default will search for the `./.blueprints` directory.

## How to use it

1. Run the program: `./target/release/code_templates`
2. Select a template
3. Insert the desired name
4. Select the target path, this will be the directory where the files will be created

## Known issues

- Only admits 1 level of encapsulation. This means that within the `templates`/`blueprints` folder, only the direct childrens will be treated as templates.
- Only admits simple templates, with a single variable.
