emcc --js-opts 0 -O0 -g4 -s WASM=0 --bind -o imgui_out.js -I ./imgui_js/imgui imgui_js/bind-imgui.cpp imgui_js/imgui/imgui.cpp imgui_js/imgui/imgui_widgets.cpp imgui_js/imgui/imgui_draw.cpp imgui_js/imgui/imgui_demo.cpp