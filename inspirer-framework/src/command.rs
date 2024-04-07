use std::{collections::HashMap, marker::PhantomData};

use clap::{ArgMatches, Command, CommandFactory, FromArgMatches};

use crate::preludes::{AppContext, AppTrait, Result};

#[async_trait::async_trait]
pub trait AppCommand<T>
where
    T: AppTrait,
{
    async fn execute(&self, context: AppContext<T>) -> Result<()>;
}

#[derive(Default)]
pub struct CommandRegister<T>
where
    T: AppTrait + 'static,
{
    pub(crate) commands:
        HashMap<&'static str, (Command, Box<dyn Fn(&ArgMatches) -> Box<dyn AppCommand<T>>>)>,
}

impl<T> CommandRegister<T>
where
    T: AppTrait + 'static,
{
    pub(crate) fn new() -> Self {
        CommandRegister {
            commands: HashMap::new()
        }
    }

    pub fn register<C>(&mut self, command: &'static str) -> &mut Self
    where
        C: AppCommand<T> + FromArgMatches + CommandFactory + 'static,
    {
        self.commands.insert(
            command,
            (
                C::command().name(command),
                Box::new(|args| Box::new(C::from_arg_matches(args).unwrap())),
            ),
        );
        self
    }
}
