(function (w) {
    // 通用方法
    var G ={
        // base64
        encode:function (input) {
            var _keyStr = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
            var output = "", chr1, chr2, chr3, enc1, enc2, enc3, enc4, i = 0;
            //input = this.utf8(input);
            while (i < input.length) {
                chr1 = input.charCodeAt(i++);
                chr2 = input.charCodeAt(i++);
                chr3 = input.charCodeAt(i++);
                enc1 = chr1 >> 2;
                enc2 = ((chr1 & 3) << 4) | (chr2 >> 4);
                enc3 = ((chr2 & 15) << 2) | (chr3 >> 6);
                enc4 = chr3 & 63;
                if (isNaN(chr2)) {
                    enc3 = enc4 = 64;
                } else if (isNaN(chr3)) {
                    enc4 = 64;
                }
                output = output + _keyStr.charAt(enc1) + _keyStr.charAt(enc2)
                    + _keyStr.charAt(enc3) + _keyStr.charAt(enc4);
            }
            return output;
        }
    };
    var ng = w.navigator,d = document, de = d.documentElement, loc = w.location;
    var pack = {
        // 获取浏览器类型
        user_agent: ng.userAgent,
        // 获取屏幕宽度
        screen_width: de.offsetWidth,
        // 获取屏幕高度
        screen_height: de.offsetHeight,
        // 获取来源地址
        referer: d.referrer,
        // 获取路径
        path: loc.pathname,
        // 获取查询条件
        query: loc.search,
        // 获取标题
        title: d.title
    };
    // 获取脚本信息
    var ss = d.getElementsByTagName('SCRIPT');
    var matches = /([^\/]+\/\/[^\/]+)\/[^:\.]+\.js\?([^&]+)&callback=(.+)$/
        .exec(ss[ss.length-1].src);
    // 域名路径
    var prefix_path = matches[1];
    // 域名Hash值
    pack.domain_hash = matches[2];
    // 回调
    pack.callback = matches[3];
    // 拼接参数
    var pack_str = "";
    for(var a in pack)pack_str+="&"+a+"="+encodeURIComponent(pack[a]);
    // 上报数据
    var push = document.createElement("SCRIPT");
    push.src = prefix_path+"/stat/site_po?hash="+pack.domain_hash
        +"&pack="+G.encode(pack_str.substring(1));
    ss[0].parentNode.append(push);
    //console.log("--",G.encode(pack_str));
})(window);