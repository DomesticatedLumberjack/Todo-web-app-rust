mod components;

use leptos::*;
use leptos_router::*;
use model::response::{user::User};
use components::login::Login;
use components::list::List;
use components::navbar::NavBar;
use components::footer::Footer;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(
        |cx| {
            let (user, set_user) = create_signal(cx, None::<User>);
            
            provide_context(cx, user);
            provide_context(cx, set_user);

            let is_logged_in = move |_| user().is_some();

            view!{ cx, 
                <Router>
                    <nav>
                        <NavBar/>
                    </nav>
                    <main>
                        <Routes>
                            <Route 
                                path="/" 
                                view=|cx| view!{cx, <Login/>} 
                            />
                            <ProtectedRoute 
                                path="/list" 
                                redirect_path="/" 
                                condition=is_logged_in 
                                view=|cx| view!{cx, <List/>}
                            />
                        </Routes>
                    </main>
                    <Footer/>
                </Router>
            }
        }
    )
}