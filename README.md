# task-add-json

I have a bunch of scripts that work with
[Taskwarrior](https://taskwarrior.org/). Because of that, I want to be able to
parse them the way `task add` does, but I want to modify the tasks before adding
them to the database. This repo parses arguments the same way that `task add`
does, but it returns JSON appropriate for feeding into `task import` after
further processing.

For example:

```sh
$ task add buy milk +groceries pri:M
{
  "description": "buy milk",
  "priority": "M",
  "tags": [
    "groceries"
  ]
}
```

It doesn't parse dates (since `task import` can do that just fine) but it'll
create the correct structure for you to whatever processing you need.
