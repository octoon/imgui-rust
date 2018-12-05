
extern crate imgui_sys;


#[derive(Debug, Clone)]
pub struct ImguiIO{
    imgui_io:*mut imgui_sys::ImGuiIO
}

#[derive(Debug, Clone)]
pub struct ImDrawData{
    imgui_draw_data:*mut ImDrawData
}

impl ImDrawData{
    pub fn iterate_draw_lists(&self, callback:fn(*mut ImDrawList)->()){
    }
}

#[derive(Debug, Clone)]
pub struct ImDrawList{
    draw_list:*mut ImDrawList
}

impl ImDrawList{
    pub fn vtx_buffer(&self) -> &imgui_sys::ImVector<imgui_sys::ImDrawVert>{

    }

    pub fn idx_buffer(&self) -> &imgui_sys::ImVector<imgui_sys::ImDrawIdx>{
        
    }

    pub fn iterate_draw_cmds(&self, callback:fn(*const ImDrawList)->()){
    }
}

#[derive(Debug, Clone)]
pub struct Imgui{
}


impl Imgui{
    pub fn new() -> Imgui{
        Imgui{
        }
    }

    #[allow(non_snake_case)]
    pub fn IMGUI_VERSION(&self) -> String{
        return String::from("");
    }

    #[allow(non_snake_case)]
    pub fn IMGUI_CHECKVERSION(&self){
    }

    pub fn create_context(&self){
        
    }

    pub fn new_frame(&self){
        unsafe{
            imgui_sys::igNewFrame();
        }
    }

    pub fn end_frame(&self){
        unsafe{
            imgui_sys::igEndFrame();
        }
    }

    pub fn render(&self){
        unsafe{
            imgui_sys::igRender();
        }
    }

    pub fn get_io(&self) -> ImguiIO{
        let ret = ImguiIO{
            imgui_io:0 as _
        };
        unsafe{
            let io = imgui_sys::igGetIO();
            ret.imgui_io = io;
        }
        ret
    }

    pub fn get_draw_data(&self) -> ImDrawData{

    }

    pub fn style_colors_dark(&self){

    }

    #[allow(non_snake_case)]
    pub fn IM_ASSERT(&self, cond:bool){

    }
}