
use std::{collections::HashMap};
use crate::tsp::utils::alias::{Step, ActionId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::prelude::*;
pub trait DotGraph {
    fn to_dot(&self) -> String;
    fn to_dot_with_param(&self, param : String) -> String;
}

impl PathGraph {

    pub fn to_png(&self, name : String, path : Option<String>) -> std::io::Result<()> {
        let mut path_txt = "./test_visual/graphs".to_string();
        if path.is_some() {
            path_txt = path.unwrap();
        }

        let txt = self.to_dot();

        let input_file = format!("{path}/{name}.dot",path=path_txt, name=name);
        let output_file = format!("{path}/{name}.png",path=path_txt, name=name);
        let path_input_file = Path::new(&input_file);
        println!("{:?}",&path_input_file.to_str());
        
        let mut file_dot = File::create(&path_input_file);
        /* 
        if file_dot.is_ok() {
            file_dot.unwrap().write(txt.as_bytes());
        }*/

        Ok(())
    }
    /*
    function to_png(graph :: Graph, name :: String, path :: String = "./test_visual/graphs")
    txt = Graphviz.to_dot(graph)
    input_file = "$path/$name.dot"
    output_file = "$path/$name.png"
    open(input_file, "w") do io
        print(io, txt)
    end
    run(`dot -Tpng $input_file -o $output_file`)
end
    */

    pub(super) fn _get_list_actions_id_by_step(&self) -> HashMap<Step, Vec<ActionId>> {
        let mut dict_lines : HashMap<Step, Vec<ActionId>> = HashMap::new();
        let list_lines = self.table_lines.dict().to_list();

        for (step, nodes) in list_lines {
            let mut list_actions_id :  Vec<ActionId> = Vec::new();
            for node in nodes {
                list_actions_id.push(node.action_id())
            }
            dict_lines.insert(step,list_actions_id);
        }

        return dict_lines;
    }

    pub(super) fn _to_dot_action(&self, action_id : ActionId) -> String {
        let mut dot_txt ="subgraph cluster_act_$action_id {\n".to_string();
        dot_txt += "label = \"Action: $action_id\"";

        let table_nodes = self.table_nodes_by_action.get(&action_id).unwrap();
        let list_nodes = table_nodes.to_list();

        for (node_id, node) in list_nodes {
            let color = node.color();
            let sons_nodes_txt = "".to_string();
            let parents_nodes_txt = "".to_string();
            let owners_html = "".to_string();

            let id_txt = node_id.to_dot_with_param("_".to_string());
            let node_id_txt = node_id.to_dot_with_param(".".to_string());
            let name = format!("step_{id_txt}", id_txt = id_txt);
            let mut node_label_html = format!("<{color}<BR /><FONT POINT-SIZE=\"8\">ID: {node_id_txt}</FONT>", color=color, node_id_txt=node_id_txt);
            node_label_html += &format!(r#""<BR /><FONT POINT-SIZE=\"8\">Parents: {parents_nodes_txt}</FONT>"#,parents_nodes_txt=parents_nodes_txt);
            node_label_html += &format!(r#"<BR /><FONT POINT-SIZE=\"8\">Sons: {sons_nodes_txt}</FONT>"#,sons_nodes_txt=sons_nodes_txt);
            node_label_html += &owners_html;
            node_label_html += ">";
            let node_definition =  format!("{name} [label={node_label_html}]\n", name=name, node_label_html=node_label_html);
        
            dot_txt += &node_definition;
        }
        dot_txt += "}";

        return dot_txt;
    }
}

/*
function action_id_to_dot(graph :: Graph, action_id :: ActionId) :: String
    dot_txt ="subgraph cluster_act_$action_id {\n"
    dot_txt *= "label = \"Action: $action_id\"";
    for (NodeId, node) in graph.table_nodes[action_id]
        color = node.color
        sons_nodes_txt = sons_to_text(node)
        parents_nodes_txt = parents_to_text(node)
        owners_html = owners_to_text(graph, node)

        id_txt = NodeIdentity.to_string(node.id,"_")
        node_id_txt = NodeIdentity.to_string(node.id)
        name = "step_$(id_txt)"
        node_label_html = "<$color<BR /><FONT POINT-SIZE=\"8\">ID: $node_id_txt</FONT>"
        node_label_html *= "<BR /><FONT POINT-SIZE=\"8\">Parents: $parents_nodes_txt</FONT>"
        node_label_html *= "<BR /><FONT POINT-SIZE=\"8\">Sons: $sons_nodes_txt</FONT>"
        node_label_html *= owners_html
        node_label_html *= ">"
        node_definition =  "$name [label=$node_label_html]"

        dot_txt *= node_definition*"\n"
    end
    dot_txt *= "}"

    return dot_txt
end */

impl DotGraph for NodeId {
    fn to_dot_with_param(&self, param : String) -> String { 
        return format!("{}{}{}",self.action_id(),param, self.action_parent_id());
    }

    fn to_dot(&self) -> String { "".to_string() }
}

impl DotGraph for PathGraph {
    fn to_dot_with_param(&self, param : String) -> String { 
        "".to_string()
    }

    fn to_dot(&self) -> String {
        let mut result = "".to_string();

        for (step, list_actions) in self._get_list_actions_id_by_step() {

            let mut def_nodes = "".to_string();
            for action_id in list_actions {
                let action_dot = self._to_dot_action(action_id);
                def_nodes = def_nodes + &action_dot + "\n";
            }

            let content = format!(r#"
            subgraph cluster_{step} {{
            style=filled;
            color=lightgrey;
            node [style=filled,color=white];
            {def_nodes}
            fontsize="10"
            label = "Line {step}";
            }}"#, step = step, def_nodes=def_nodes);

            result = result + &content;
        }
        /*    for (step, list_actions) in get_list_actions_id_by_step(graph)
        def_nodes = ""
        for action_id in list_actions
            def_nodes *= action_id_to_dot(graph, action_id) *"\n"
        end

        dot_content_txt *= "
        subgraph cluster_$step {
        style=filled;
        color=lightgrey;
        node [style=filled,color=white];
        $def_nodes
        fontsize=\"10\"
        label = \"Line $step\";
        }\n"
    end */

        
        return result;
    }


    /*
    function get_list_actions_id_by_step(graph :: Graph) :: Dict{Step, Array{ActionId, 1}}
    dist_lines = Dict{Step, Array{ActionId, 1}}()
    for (step, nodes) in graph.table_lines
        list_actions_id = Array{ActionId, 1}()
        for node_id in nodes
            node = PathGraph.get_node(graph, node_id)
            push!(list_actions_id, node.action_id)
        end

        dist_lines[step] = list_actions_id
    end

    return dist_lines
end */
}

