function load_base_svg(container_id, svg_string) {
    let target = document.getElementById(container_id);
    target.innerHTML = svg_string;
}

function apply_animation(diffs) {
    for (const diff of diffs) {
        console.log("Diff:", diff);
        switch (diff.action) {
            case "remove":
                console.log("removing");
                let remove_element = document.getElementById(diff.id);
                remove_element.remove();
                break;
            case "add":
                console.log("adding");
                let parent_element = document.getElementById(diff.parent_id);
                const children = parent_element.children;
                let insert_index = 0;
                if (diff.prev_child_id !== undefined) {
                    for (let index = 0; index < children.length; ++index) {
                        if (parent_element.children[index].id === diff.prev_child_id) {
                            insert_index = index + 1;
                            break;
                        }
                    }
                }
                // If we are inserted at the end, we pass null
                if (insert_index === children.length) {
                    insert_index = null;
                }
                parent_element = SVG(parent_element);
                let new_element = SVG(diff.svg);
                parent_element.add(new_element, insert_index);
                new_element.attr('opacity', 0);
                new_element.animate(2000, 0, 'now').attr('opacity', 1);
                break;
            case "change":
                console.log("change");
                let dom_element = document.getElementById(diff.id);
                let element = SVG(dom_element);
                for (remove of diff.removes) {
                    dom_element.removeAttribute(remove.prop);
                }
                for (change of diff.changes) {
                    if (change.prop === "transform") {
                        element.animate(2000, 0, 'now').transform(change.value);
                    } else {
                        element.animate(2000, 0, 'now').attr(change.prop, change.value);
                    }
                }
                for (add of diff.adds) {
                    if (add.prop === "transform") {
                        element.animate(2000, 0, 'now').transform(add.value);
                    } else {
                        element.animate(2000, 0, 'now').attr(add.prop, add.value);
                    }
                }
                break;
            case "change_text":
                let text_element = document.getElementById(diff.id);
                element.textContent = diff.new_text
                break;
        }

    }
}
