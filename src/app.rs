use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" />
        <Link href="https://fonts.googleapis.com/css2?family=Nunito+Sans:ital,opsz,wdth,wght,YTLC@0,6..12,75..125,600,440..540;1,6..12,75..125,600,440..540&display=swap" rel="stylesheet" />

        <Router>
            <Routes>
                <Route path="/" view=  move || view! { <Home/> }/>
                <Route path="/users" view=Users/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let navigate = leptos_router::use_navigate();

    view! {
        <Title text="Guardiann"/>
        <main>
            <div class="bg-gradient-to-tl from-[#FFF] to-[#b9b9b9] text-white font-mono flex flex-col min-h-screen h-full">
                <div class="flex flex-row-reverse flex-wrap m-auto pt-[197px] pb-[137px]">
                    <div class="flex flex-col items-center justify-center bg-white rounded-3xl stroke-[#b9b9b9] p-20 h-fit">
                        <img src="/logo.jpg" alt="logo" class="w-[200px] h-[62px] mb-8 object-cover" />
                        <div class="flex flex-col items-start justify-center mt-[37px] gap-3">
                            <label for="email" class="text-[#202224] font-bold text-lg -tracking-[0.06px]">"Email address:"</label>
                            <input type="email" placeholder="guardian@gmail.com" class="bg-[#f1f4f9] stroke-[#d8d8d8] outline-1 outline-[#4880ff] transition-all text-[#202224] rounded-lg w-[516px] h-14 px-4 text-lg font-semibold" />
                        </div>
                        <div class="flex flex-col items-start justify-center mt-10 gap-3">
                            <label for="password" class="text-[#202224] font-bold text-lg -tracking-[0.06px]">"Password:"</label>
                            <input type="password" placeholder="********" class="bg-[#f1f4f9] stroke-[#d8d8d8] rounded-lg w-[516px] outline-1 outline-[#4880ff] transition-all text-[#202224] h-14 px-4 text-lg font-semibold" />
                        </div>
                        <button 
                            on:click=move |_| navigate("/users", Default::default())
                            class="mt-12 bg-[#4880ff] text-white w-full rounded-lg h-14 text-center font-bold text-[20px] -tracking-[0.07px]"
                        >
                            "Sign In"
                        </button>
                        <div class="flex items-center justify-between w-full mt-4">
                            <div class="flex items-center justify-center gap-3 w-fit">
                                <input type="checkbox" class="w-5 h-5 rounded-md stroke-[#d8d8d8] cursor-pointer" />
                                <label for="remember" class="text-[#202224] font-semibold text-lg -tracking-[0.06px] opacity-60">"Remember me"</label>
                            </div>
                            <a href="#" class="text-[#202224] font-semibold text-lg -tracking-[0.06px] opacity-60">"Forgot your password?"</a>
                        </div>
                        <a href="#" class="text-[#013664] font-semibold text-base -tracking-[0.06px] underline underline-offset-[4px] mt-4">"privacy policy"</a>
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
fn Users() -> impl IntoView {
    let navigate = leptos_router::use_navigate();
    
    view! {
        <div 
            class="w-screen h-screen bg-[#F5F6FA] flex"
        >
            <aside class="w-[240px] h-screen flex flex-col bg-white items-center justify-start py-4">
                <img src="/logo.jpg" alt="logo" class="w-[127px] h-[40px] object-cover mb-6" />
                <nav class="flex flex-col items-start justify-center gap-4 mt-8 w-full">
                    <div class="flex flex-col w-full items-center  pl-10">
                        <div class="w-full items-center justify-start mx-auto flex gap-4 h-[50px]">
                            <img src="/dashboard.svg" alt="search" class="w-[15px] h-[15px]" />
                            <a href="#" class="text-[#202224] font-semibold text-sm tracking-[0.03px]">"Dashboard"</a>
                        </div>
                        <div class="w-full items-center justify-start mx-auto flex gap-4 h-[50px]">
                            <img src="/sheet.svg" alt="sheet" class="w-[15px] h-[15px]" />
                            <a href="#" class="text-[#202224] font-semibold text-sm tracking-[0.03px]">"Spreadsheets"</a>
                        </div>
                    </div>
                    
                    <div class="flex flex-col w-full items-center mt-72">
                        <hr class="w-full bg-[#d8d8d8] stroke-[#e8e8e8]" />
                        <div class="w-full items-center justify-start mx-auto flex gap-4 h-[50px]  pl-10">
                            <img src="/settings.svg" alt="settings" class="w-[15px] h-[15px]" />
                            <a href="#" class="text-[#202224] font-semibold text-sm -tracking-[0.06px]">"Settings"</a>
                        </div>
                        <button 
                            on:click=move |_| navigate("/", Default::default())
                            class="w-full items-center justify-start mx-auto flex gap-4 h-[50px]  pl-10 cursor-pointer"
                        >
                            <img src="/logout.svg" alt="logout" class="w-[15px] h-[15px]" />
                            <a href="#" class="text-[#202224] font-semibold text-sm -tracking-[0.06px]">"Logout"</a>
                        </button>
                    </div>
                </nav>
            </aside>
            <div class="w-full flex flex-col">
                <header class="w-full h-[70px] bg-white flex items-center justify-start px-[30px] py-3 gap-6">
                    <img src="/hamburguer.svg" alt="hamburguer" class="w-[25px] h-[25px]" />
                    <form class="w-[388px] h-[38px] rounded-2xl stroke-[#d5d5d5] bg-[#f5f6fa] flex items-center gap-[13.5px] relative">
                        <div class="absolute inset-y-0 start-0 flex items-center px-3 pointer-events-none">
                            <img src="/search.svg" alt="search" class="w-[15px] h-[15px]" />
                        </div>
                        <input 
                            type="search" 
                            id="search" 
                            class="block w-full p-2 px-10 text-sm text-[#202224] border border-gray-300 rounded-2xl bg-transparent outline-none" 
                            placeholder="Search" 
                        />
                    </form>
                </header>
                
                <main class="p-[30px] h-full w-full">
                    <h1 class="font-bold text-[32px] -tracking-[0.16px] text-[#202224]">"Spreadsheets"</h1>
                </main>

            </div>
        </div>
    }
}