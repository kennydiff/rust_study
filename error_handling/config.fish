# function bind_bang
#     switch (commandline -t)[-1]
#         case "!"
#             commandline -t $history[1]; commandline -f repaint
#         case "*"
#             commandline -i !
#     end
# end

# function bind_dollar
#     switch (commandline -t)[-1]
#         case "!"
#             commandline -t ""
#             commandline -f history-token-search-backward
#         case "*"
#             commandline -i '$'
#     end
# end

# function fish_user_key_bindings
#     bind ! bind_bang
#     bind '$' bind_dollar
# end

if status is-interactive
    # Commands to run in interactive sessions can go here
    set -gx LANG "en_US.utf-8"
    set proxy_host 127.0.0.1:7890
    # set proxy_auth false
    
    # Kenny@20220611 默认开启全局代理...or 手动运行proxy开启全局代理/noproxy关闭全局代理
    # Kenny@20220611 检查代理是否生效: set -x |grep proxy
    # proxy
    
    alias cdl='cd ~/Downloads'  # Kenny@20220611 设置cdd为快速切换路径到~/Downloads的命令
    alias crs='cd ~/nCloud/,V2FS/1.RustPRJ/rust_study'  # Kenny@20220611 设置cdd为快速切换路径到~/rust_study的命令
    
    # Kenny@20220611 这三个函数是!! / !$ 这两个快捷指令(上一条指令/参数)的fish实现
    # bind_bang
    # bind_dollar
    # fish_user_key_bindings

end