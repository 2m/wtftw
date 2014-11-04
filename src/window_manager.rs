use core::Workspaces;
use config::Config;
use layout::LayoutManager;
use window_system::Rectangle;
use window_system::Window;
use window_system::WindowSystem;

pub type ScreenDetail = Rectangle;

pub struct WindowManager {
    workspaces: Workspaces
}

impl WindowManager {
    pub fn new(window_system: &WindowSystem, config: &Config) -> WindowManager {
        WindowManager {
            workspaces: Workspaces::new(String::from_str("Tall"), 
                                        config.tags.clone(), 
                                        window_system.get_screen_infos())
        }
    }

    pub fn is_window_managed(&self, window: Window) -> bool {
        self.workspaces.contains(window)
    }

    pub fn view(&mut self, window_system: &mut WindowSystem, index: uint, config: &Config) {
        self.workspaces.view(index);
        self.reapply_layout(window_system, config);
    }

    pub fn reapply_layout(&mut self, window_system: &mut WindowSystem, config: &Config) {
        let screen = &self.workspaces.current;
        let workspace = &screen.workspace;
        let layout = LayoutManager::get_layout(workspace.layout.clone());
        let window_layout = layout.apply_layout(screen.screen_detail, &workspace.stack); 

        for &(win, Rectangle(x, y, w, h)) in window_layout.iter() {
            debug!("Show window {}", win);
            window_system.show_window(win);
            window_system.resize_window(win, w - config.border_width * 2, h - config.border_width * 2);
            window_system.move_window(win, x, y);
            window_system.set_window_border_width(win, config.border_width);
        }

        for workspace in self.workspaces.hidden.iter() {
            match workspace.stack {
                Some(ref s) => {
                    for &win in s.integrate().iter() {
                        window_system.hide_window(win);
                    }
                }
                _ => ()
            }
        }
    }

    pub fn manage(&mut self, window_system: &mut WindowSystem, window: Window, config: &Config) {
        self.workspaces.current.workspace.add(window);
        self.reapply_layout(window_system, config);   
        debug!("managing window \"{}\" ({})", window_system.get_window_name(window), window);
    }

    pub fn unmanage(&mut self, window_system: &mut WindowSystem, window: Window, config: &Config) {
        if self.workspaces.contains(window) {
            debug!("unmanaging window {}", window);
            self.workspaces.delete(window);
            self.reapply_layout(window_system, config);
        }
    }
}