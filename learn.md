

cargo new --vcs=none crates/web-server

The database
    CREATE USER application_user;
    
    迁移数据库：
        先 export DATABASE_URL=postgres://postgres:password@localhost:5432/postgres?sslmode=disable
        创建迁移：dbmate new user_tables
        查看迁移状态：dbmate status
        执行迁移：dbmate up

cargo install clorinde  安装 clorinde，clorinde是一个代码生成器，它接收小型 SQL 片段并将其转换为 Rust 函数

cargo init --lib --vcs none crates/db
    - cargo init，类似cargo new，但只针对当前已存在目录
    - --vcs none 不初始化版本控制系统（Version Control System），默认情况下，cargo init 会初始化 Git 仓库

-----------------------------------------------------------------------------------------------------------------------
使用clorinde将SQL转化为Rust代码
    clorinde live -q ./crates/db/queries/ -d crates/clorinde --serialize true   -> --serialize true 可以解决错误

cargo init --lib crates/web-pages
    cargo add dioxus@0.6 --no-default-features -F macro,html,signals  => dioxus = { version = "0.6", default-features = false, features = ["macro", "html", "signals"] }

cargo init --lib crates/web-assets
    cd crates/web-assets && tailwindcss-extra -i ./input.css -o ./dist/tailwind.css   =>  生成tailwind.cass
    解决错误：
        - selected_item:SideBar::Users,
        - stylesheets: vec![web_assets::files::tailwind_css.name.to_string()],

