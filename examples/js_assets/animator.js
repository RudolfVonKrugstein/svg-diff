async function load_base_svg(container_id, svg) {
    if (svg === undefined) {
        let response = await fetch('./svg');
        svg = await response.text();
    }
    let target = document.getElementById(container_id);
    target.innerHTML = svg;
}

function html_to_svg_element(html) {
    let template = document.createElementNS("http://www.w3.org/2000/svg",'svg');
    html = html.trim();
    template.innerHTML = html;
    return template.firstChild;
}

function apply_animation(diffs) {
    for (const diff of diffs) {
        console.log("Diff:", diff);
        switch (diff.action) {
            case "remove":
                console.log("removing");
                d3.select('#' + diff.id)
                    .attr("opacity", 1.0)
                    .transition()
                    .duration(1000)
                    .attr("opacity", 0.0).remove();
                break;
            case "add":
                console.log("adding");
                let new_element = html_to_svg_element(diff.svg);
                let parent_element = document.getElementById(diff.parent_id);
                if (diff.next_child_id !== undefined) {
                    parent_element.insertBefore(
                        new_element,
                        document.getElementById(diff.next_child_id)
                    );
                } else {
                    parent_element.appendChild(new_element);
                }
                let new_el = d3.select('#' + diff.id);
                new_el.attr('opacity', 0.0);
                new_el.transition()
                    .duration(1000)
                    .attr('opacity', 1.0);
                // KUTE.fromTo(
                //     '#' + diff.id,
                //     {opacity:0},
                //     {opacity:1}
                // ).start();
                break;
            case "change":
                console.log("change");
                let el = d3.select('#' + diff.id);
                let dom_element = document.getElementById(diff.id);
                for (remove of diff.removes) {
                    el.attr(remove, null)
                }

                let anim = el.transition().duration(1000);
                for (change of diff.changes) {
                    anim.attr(change.prop, change.end);
                }
                for (change of diff.adds) {
                    anim.attr(change.prop, change.value);
                }
                break;
            case "change_text":
                let text_element = document.getElementById(diff.id);
                element.textContent = diff.new_text
                break;
        }

    }
}
