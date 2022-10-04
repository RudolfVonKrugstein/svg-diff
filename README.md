# svg-diff - How to get from one SVG to the other?

The purpose of this library is to find the "diff" between 2 SVGs in the form
of simple operations that have to be executed to get from one SVG to the other.

## Intended usage

The original goal is to generate animations (using some animation library
like [d3.js]) which transition from one SVG to the other.
This can be useful i.E. when making presentation with [revealjs] and
generating diagrams with [plantUML]. One can generate "intermediate" Diagrams
and transition between using the diffs genrate by this tool.

Of course other usages are possible, but do not have been considered by
the author of this library.

## Formats

For the diff between 2 SVGs the following is generated:

* A "base" SVG which is a modified version of the first SVG and should be used
  to apply the diff to. The modifications are mainly:
  * Give an ID to very element that has to be touched during the transition.
    These IDs are also used in the diff.
  * Change some attributes to be better compatible with animation libraries.
    For example colors are converted to the hex format.
* A json with the diffs.
  ```json
  [{
    "action": "add",
    "id": "sjffk-5",
    "svg": "<circle id\"sjffk-1\" ...></circle>",
    "parent_id": "sjffk-3",
    "prev_child_id": "sjffk-1",
    "next_child_id": "sjffk-8"
  },{
    "action": "remove",
    "id": "sjffk-2"
  },
  {
     "action": "change",
     "changes": {
       "removes": ["opacity"],
       "adds": [{"prop": "fill", "value": "#FF0000"}],
       "changes": [{"prop": "stroke", "value": "#000000"}]
     }
  }]
  ```

## Examples

In the examples folder there are 2 examples that demonstrate how to make
SVG animations with this.

Start the examples with:

```bash
cargo run --example <exmaple-name>
```

and than open http://localhost:8080 in your web browser.

[d3.js]: https://d3js.org/
[revealjs]: https://revealjs.com/
[plantUML]: https://plantuml.com/
