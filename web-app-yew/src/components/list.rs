use yew::prelude::*;

use crate::model::db::{task::Task};

#[derive(Properties, PartialEq)]
pub struct Props{
    pub list_items: Vec<Task>,
    pub update_task: Callback<Task>
}

#[function_component]
pub fn List(props: &Props) -> Html{
    let task_items = props.list_items.clone();
    let update_task= props.update_task.clone();

    html!{
    <table>
    {
        task_items.into_iter().map(|task|{
            let display_task = task.clone();
            let update_task = update_task.clone();
            html!{
                <tr>
                    <td>
                        <input type={"checkbox"} checked={task.complete} oninput={move |_| update_task.emit(task.clone())}/>
                    </td>
                    <td>
                        {display_task.description}
                    </td>
                </tr>
            }
        }).collect::<Html>()
    }
    </table>
    }
}