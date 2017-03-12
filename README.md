# mirrors

Easily add multiple remotes to Git repositories

### Usage

Create `mirrors.json` in repository root:
```json
{
  "primary": "https://github.com/myusername/myproject.git",
  "mirrors": [
    "https://myusername@bitbucket.org/myusername/myproject-mirror1.git",
    "https://myusername@bitbucket.org/myusername/myproject-mirror2.git"
  ]
}
```
`primary` will be push+fetch, `mirrors` will be fetch only.

Add this repository to `$PATH` and run:
```shell
mirrors
```
