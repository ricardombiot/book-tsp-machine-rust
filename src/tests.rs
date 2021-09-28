#[cfg(test)]
mod hello;

pub mod actions {
    #[cfg(test)]
    mod database_actions_test;
}

pub mod machine {
    pub mod components {
        #[cfg(test)]
        mod graf_test;
    }
}