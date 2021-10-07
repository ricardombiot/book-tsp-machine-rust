
use crate::tsp::actions::action;
use crate::tsp::utils::alias::{Color, Step, ActionId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::pathset::components::nodes::node_id::NodeId;

impl PathGraph {

    pub fn up(&mut self, color : Color, action_id : ActionId){
        if self.valid {
            //println!("Step{}::UP::Color:{} - Act.Id{}.{}", self.next_step, &color, &action_id, self.action_parent_id.unwrap());
            // # $ O(N^4) $ deleting all nodes
            self._delete_node_by_color(&color);

            /*
            # Maximum theoretical $ O(N^{10}) $
            # Most probable less than N stages: $ O(N^8) $
            # $ O(Stages) * O(N^7) $
            */
            self.review_owners_all_graph();

            //# $ O(N^3) $
            self._make_up(&color, &action_id)
        }
    }

    pub fn make_up(&mut self, color : Color, action_id : ActionId){
        self._make_up(&color, &action_id);
    }
    fn _make_up(&mut self, color : &Color, action_id : &ActionId){
        if self.valid {
            let last_step = self.next_step - (1 as Step);

            let node = self._new_node(color.clone(), action_id.clone());
            let node_id = node.id().clone();
            
            self._add_line();
            self.action_parent_id = Some(action_id.clone());
            
            //# $ O(N^3) $
            self._add_node(node);
        
            //# $ O(N) $
            self._add_all_nodes_last_step_as_parents(last_step, node_id);
        
            self.next_step += 1 as Step;
        }
    }

    fn _add_all_nodes_last_step_as_parents(&mut self, last_step: Step, node_son_id : NodeId){
        //# $ O(N) - Origin - Itself $ then $ O(N-2) $ parents by node
        let set_nodes = self.table_lines.get(&last_step).unwrap().clone();
        for node_parent_id in set_nodes.iter(){
            self._add_edge(node_parent_id, &node_son_id);
        }
    }

    fn _add_edge(&mut self, node_parent_id: &NodeId, node_son_id : &NodeId){
        let action_parent_id = node_parent_id.action_id();
        let node_parent = self.table_nodes_by_action.get_node_mut(&action_parent_id, node_parent_id).unwrap();

        node_parent.add_son_id(&node_son_id);

        let action_son_id = node_son_id.action_id();
        let node_son = self.table_nodes_by_action.get_node_mut(&action_son_id, &node_son_id).unwrap();

        node_son.add_parent_id(node_parent_id);

        self.table_edges.build_edge(node_parent_id, node_son_id);
    }

}


/*# Maximum theoretical $ O(N^10) $
# Most probable less than: $ O(N^8) $
function up!(graph :: Graph, color :: Color, action_id :: ActionId)
    # $ O(N^4) $ deleting all nodes
    delete_node_by_color!(graph, color)

    # Maximum theoretical $ O(N^{10}) $
    # Most probable less than N stages: $ O(N^8) $
    # $ O(Stages) * O(N^7) $
    review_owners_all_graph!(graph)

    # $ O(N^3) $
    make_up!(graph, color, action_id)
end

# $ O(N^3) $
function make_up!(graph :: Graph, color :: Color, action_id :: ActionId)
    last_step = graph.next_step - Step(1)
    node = new_node(graph, color, action_id)
    graph.action_parent_id = action_id
    add_line!(graph)
    # $ O(N^3) $
    add_node!(graph, node)

    # $ O(N) $
    add_all_nodes_last_step_as_parents!(graph, node, last_step)

    graph.next_step += Step(1)
end

# $ O(N) $
function add_all_nodes_last_step_as_parents!(graph :: Graph, node :: Node, last_step :: Step)
    # $ O(N) - Origin - Itself $ then $ O(N-2) $ parents by node
    for parent_id in graph.table_lines[last_step]
        add_edge!(graph, parent_id, node.id)
    end
end

function add_edge!(graph :: Graph, origin_id :: NodeId, destine_id :: NodeId)
    if have_node(graph, origin_id) && have_node(graph, destine_id)
        node_origin = get_node(graph, origin_id)
        node_destine = get_node(graph, destine_id)

        edge = PathEdge.build!(node_origin, node_destine)

        graph.table_edges[edge.id] = edge
    end
end
*/