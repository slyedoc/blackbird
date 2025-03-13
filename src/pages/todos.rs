use crate::prelude::*;
use leptos::prelude::*;

#[component]
pub fn Todos() -> impl IntoView {
    let add_todo = ServerMultiAction::<AddTodo>::new();
    let delete_todo = ServerAction::<DeleteTodo>::new();
    let submissions = add_todo.submissions();

    // list of todos is loaded from the server in reaction to changes
    let todos = Resource::new(
        move || (add_todo.version().get(), delete_todo.version().get()),
        move |_| get_todos(),
    );

    view! {
      <h1 class="mt-10 text-center text-2xl/9 font-bold tracking-tight">"Todos"</h1>

      <div>
        <MultiActionForm action=add_todo>
          <div>
            <label for="title" class="block text-sm/6 font-medium text">
              "Add a Todo"
            </label>
            <div class="mt-2 flex">
              <input type="text" name="title" required class="flex-1 input-primary" />
              <button type="submit" class="btn-primary">
                "Add"
              </button>
            </div>

          </div>
        </MultiActionForm>
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
          <ErrorBoundary fallback=|errors| {
            view! { <ErrorTemplate errors=errors /> }
          }>
            {move || {
              let existing_todos = {
                move || {
                  todos
                    .get()
                    .map(move |todos| match todos {
                      Err(e) => {
                        view! { <pre class="error">"Server Error: " {e.to_string()}</pre> }
                          .into_any()
                      }
                      Ok(todos) => {
                        if todos.is_empty() {
                          view! { <p>"No tasks were found."</p> }.into_any()
                        } else {
                          todos
                            .into_iter()
                            .map(move |todo| {
                              view! {
                                <li class="flex flex-row justify-between gap-x-6 py-5">
                                  <div class="flex grow min-w-0 gap-x-4">

                                    <div class="min-w-0 flex-auto">
                                      <p class="text-sm/6 font-semibold text-primary">
                                        {todo.title}
                                      </p>
                                      <p class="text-sm/6 text-secondary">
                                        by {todo.user.unwrap_or_default().username}
                                      </p>
                                      <p class="mt-1 text-xs/5 text-secondary">
                                        Last seen {todo.created_at}
                                      </p>
                                    </div>
                                    <div class="flex flex-col justify-end items-end">
                                      <ActionForm action=delete_todo>
                                        <input type="hidden" name="id" value=todo.id />
                                        <button type="submit" class="btn-primary">
                                          <Icon icon=i::AiDeleteOutlined />
                                        </button>
                                      </ActionForm>
                                    </div>
                                  </div>
                                </li>
                              }
                            })
                            .collect_view()
                            .into_any()
                        }
                      }
                    })
                    .unwrap_or(().into_any())
                }
              };
              let pending_todos = move || {
                submissions
                  .get()
                  .into_iter()
                  .filter(|submission| submission.pending().get())
                  .map(|submission| {
                    view! {
                      <li class="flex justify-between gap-x-6 py-5">
                        <p class="text-sm/6 font-semibold text-primary">
                            {move || submission.input().get().map(|data| data.title)}
                        </p>
                      </li>
                    }
                  })
                  .collect_view()
              };
              view! {
                <ul role="list" class="divide-y divide-gray-100">
                  {existing_todos}
                  {pending_todos}
                </ul>
              }
            }}

          </ErrorBoundary>
        </Transition>
      </div>
    }
}
