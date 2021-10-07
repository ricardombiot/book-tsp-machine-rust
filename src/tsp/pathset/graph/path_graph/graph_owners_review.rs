use crate::tsp::utils::alias::{Step};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl PathGraph {

    pub fn review_owners_all_graph(&mut self) {
        self._recursive_review_owners_all_graph(1)
    }

    fn _recursive_review_owners_all_graph(&mut self, stage : u32){
        /*# Theoretical:
        # If we would execute it after remove each node $ O(N^3) * O(N^7) = O(N^10) $
        # but in the practise we execute it at least deleting all nodes of a color
        # each delete node can produce a propation deleting, and before of delete all nodes of
        # graph will detected this situation with the owners.
    
        # It is important see that the number of executions dependes of the deletes stages required
        # to be valid or invalid graph, that at the same time depends of the instance
        # but in all cases follow a polynomial function.
        # -yes, polynomial expensive function, but polynomial be-
        */
        if self.valid && self.required_review_ownwers {
            //println!("_recursive_review_owners_all_graph = Stage:({})", stage);
            self._save_max_review_stages(stage);
            //  # $ O(N^3) $
            self._rebuild_owners();
            // # $ O(N^7) $
            self._review_owners_nodes_and_relationships();

            self.required_review_ownwers= false;
            let should_nodes_to_delete = !self.nodes_to_delete.is_empty();
            if self.valid && should_nodes_to_delete {
                self.required_review_ownwers = true;
                self.apply_node_deletes();
                self._recursive_review_owners_all_graph(stage + 1);
            }
        }
    }

    fn _save_max_review_stages(&mut self, stage : u32){
        self.max_review_stages = std::cmp::max(self.max_review_stages, stage)
    }

    fn _rebuild_owners(&mut self){
        let mut owners_graph_new = self.owners_graph.empty_derive();

        let next_step: Step = self.next_step;
        //@Review if should add the -1
        //  for step in Step(0):Step(graph.next_step-1)
        for step in 0..next_step {
            if self.table_lines.have(&step){
                let set_nodes = self.table_lines.get(&step).unwrap();
                // # $ O(N^2) $  nodes by step
                for node_id in set_nodes {
                    owners_graph_new.push(node_id);
                }
            }else{
                self.valid = false;
                break;
            }
        }

        if self.valid {
            self.owners_graph = owners_graph_new;
            self._make_validation_graph_by_owners();
        }
    }


    /*
    #=
    Down to top
    1. filter_by_intersection_owners!
    2. filter_by_sons_intersection_owners!
    3. filter_by_incoherence_colors!
    4. review_sons_filtering_by_parents_interection_owners!
    $ O(N^7) $
    =#
    */
    fn _review_owners_nodes_and_relationships(&mut self){
        // # $ O(N) * O(N^2) * O(N^4) = O(N^7) $

        if self.valid && self.required_review_ownwers {
            //println!("_review_owners_nodes_and_relationships");
            let mut step = self.next_step - 1;
            let mut stop_while = false;
            //# $ O(N) steps $
            while !stop_while {
                let set_nodes = self.table_lines.get(&step).unwrap().clone();
                // # $ O(N^2) $ nodes by step
                for node_id in set_nodes {

                    //# $ O(N^3) $
                    if self._filter_by_intersection_owners(&node_id){
                        self.save_to_delete(&node_id);
                    // # $ O(N^4) $
                    }else if self._filter_by_sons_intersection_owners(&node_id){
                        self.save_to_delete(&node_id);
                    // # $ O(N^3) $
                    }else if self._filter_by_incoherence_colors(&node_id) {
                        self.save_to_delete(&node_id);
                    }
                        
                    
                    if self.valid {
                        //stop_while = true;
                        break;
                    }
                }

                // # $ O(N^6) $
                if self._continue_review() {
                    self._review_sons_filtering_by_parents_interection_owners(&step);
                }

                if step == 0 as Step || !self.valid {
                    stop_while = true;
                }else{
                    step -= 1;
                }
            }
        }
    }   

    fn _continue_review(&self) -> bool {
        return self.nodes_to_delete.is_empty() && self.valid
    }
    
}
