use hlua;
use std::sync::mpsc;


pub struct PrintBuffer {
    rx: mpsc::Receiver<String>,
    tx: mpsc::Sender<String>,
}

impl PrintBuffer {
    pub fn new_and_expose_to_lua(lua: &mut hlua::Lua, env_name: &str) -> PrintBuffer {
        let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let lua_tx = tx.clone();
        lua.set(
            "__print",
            hlua::function1(move |s: String| {
                lua_tx.send(s).unwrap();
            }),
        );
        lua.execute::<()>(&format!(
            "
            function print (...)
              for i,v in ipairs{{...}} do
                __print(tostring(v) .. \"\\t\")
              end
              __print(\"\\n\")
            end
            {env}.print = print;",
            env = env_name
        )).unwrap();
        PrintBuffer { rx: rx, tx: tx }
    }
    pub fn get_tx(&self) -> mpsc::Sender<String> {
        self.tx.clone()
    }
    pub fn get_buffer(&self) -> String {
        let mut result = String::new();
        for s in self.rx.try_iter() {
            result += &s;
        }
        return result;
    }
}
