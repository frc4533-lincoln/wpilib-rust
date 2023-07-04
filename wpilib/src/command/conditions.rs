use super::manager::*;
use std::sync::Arc;
#[derive(Clone)]
pub struct OnTrue<T>
    where T:  Fn() -> bool + Send + Sync + 'static{
    pub function: Arc<T>,
    pub last_state: bool,
}

impl<T> Condition for OnTrue<T> 
    where T:  Fn() -> bool + Send + Sync + 'static{
    fn get_condition(&mut self) -> ConditionResponse {
        let state = (self.function)();
        if state == true && self.last_state == false {
            self.last_state = state;
            return ConditionResponse::Start;
        }
        self.last_state = state;
        ConditionResponse::NoChange
    }

    fn clone_boxed(&self) -> Box<dyn Condition> {
        Box::new(
            Self{
                function: self.function.clone(),
                last_state: self.last_state.clone()
            }
        )
    }

}

impl<T> std::fmt::Debug for OnTrue<T>
    where T:  Fn() -> bool + Send + Sync + 'static{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnTrue")
            .field("last_state", &self.last_state)
            .finish()
        
    }
}



#[derive(Clone)]
pub struct WhileTrue<T>
    where T:  Fn() -> bool + Send + Sync + 'static{
    pub function: Arc<T>,
    pub last_state: bool,
}

impl<T> Condition for WhileTrue<T> 
    where T:  Fn() -> bool + Send + Sync + 'static{
    fn get_condition(&mut self) -> ConditionResponse {
        let state = (self.function)();
        if state == true && self.last_state == false {
            self.last_state = state;
            return ConditionResponse::Start;
        } else if state == true {
            self.last_state = state;
            return ConditionResponse::Continue
        } else if state == false && self.last_state == true{
            self.last_state = state;
            return ConditionResponse::Stop
        }

        self.last_state = state;
        ConditionResponse::NoChange
    }

    fn clone_boxed(&self) -> Box<dyn Condition> {
        Box::new(
            Self{
                function: self.function.clone(),
                last_state: self.last_state.clone()
            }
        )
    }

}

impl<T> std::fmt::Debug for WhileTrue<T>
    where T:  Fn() -> bool + Send + Sync + 'static{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnTrue")
            .field("last_state", &self.last_state)
            .finish()
        
    }
}