




use regex::Regex;
pub fn is_ipv4(v: String) -> bool  {
    let re_ipv4 = Regex::new(r"((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})(\.((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})){3}").unwrap();
    return re_ipv4.is_match(v.as_str());
}

pub fn is_ipv6(v: String) -> bool  {
    let re_ipv6 = Regex::new(r"^([\da-fA-F]{1,4}:){6}((25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(25[0-5]|2[0-4]\d|[01]?\d\d?)|::([\da−fA−F]1,4:)0,4((25[0−5]|2[0−4]\d|[01]?\d\d?)\.)3(25[0−5]|2[0−4]\d|[01]?\d\d?)|::([\da−fA−F]1,4:)0,4((25[0−5]|2[0−4]\d|[01]?\d\d?)\.)3(25[0−5]|2[0−4]\d|[01]?\d\d?)|^([\da-fA-F]{1,4}:):([\da-fA-F]{1,4}:){0,3}((25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(25[0-5]|2[0-4]\d|[01]?\d\d?)|([\da−fA−F]1,4:)2:([\da−fA−F]1,4:)0,2((25[0−5]|2[0−4]\d|[01]?\d\d?)\.)3(25[0−5]|2[0−4]\d|[01]?\d\d?)|([\da−fA−F]1,4:)2:([\da−fA−F]1,4:)0,2((25[0−5]|2[0−4]\d|[01]?\d\d?)\.)3(25[0−5]|2[0−4]\d|[01]?\d\d?)|^([\da-fA-F]{1,4}:){3}:([\da-fA-F]{1,4}:){0,1}((25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(25[0-5]|2[0-4]\d|[01]?\d\d?)|([\da−fA−F]1,4:)4:((25[0−5]|2[0−4]\d|[01]?\d\d?)\.)3(25[0−5]|2[0−4]\d|[01]?\d\d?)|([\da−fA−F]1,4:)4:((25[0−5]|2[0−4]\d|[01]?\d\d?)\.)3(25[0−5]|2[0−4]\d|[01]?\d\d?)|^([\da-fA-F]{1,4}:){7}[\da-fA-F]{1,4}|:((:[\da−fA−F]1,4)1,6|:)|:((:[\da−fA−F]1,4)1,6|:)|^[\da-fA-F]{1,4}:((:[\da-fA-F]{1,4}){1,5}|:)|([\da−fA−F]1,4:)2((:[\da−fA−F]1,4)1,4|:)|([\da−fA−F]1,4:)2((:[\da−fA−F]1,4)1,4|:)|^([\da-fA-F]{1,4}:){3}((:[\da-fA-F]{1,4}){1,3}|:)|([\da−fA−F]1,4:)4((:[\da−fA−F]1,4)1,2|:)|([\da−fA−F]1,4:)4((:[\da−fA−F]1,4)1,2|:)|^([\da-fA-F]{1,4}:){5}:([\da-fA-F]{1,4})?|([\da−fA−F]1,4:)6:|([\da−fA−F]1,4:)6:").unwrap();
    return re_ipv6.is_match(v.as_str());
}

pub fn vali_ipv4(v: String) -> Result<(), String> {
    let res = is_ipv4(v);
    if res {
        return Ok(())
    }else{
        return Err(String::from("Invalid ipv4 format"))
    }
}

pub fn vali_ipv6(v: String) -> Result<(), String> {
    let res = is_ipv6(v);
    if res {
        return Ok(())
    }else{
        return Err(String::from("Invalid ipv6 format"))
    }
}

pub fn vali_ip(v: String) -> Result<(), String> {
    let res = is_ipv4(v.clone()) || is_ipv6(v);
    if res {
        return Ok(())
    }else{
        return Err(String::from("Invalid ipv6 format"))
    }
}

pub fn vali_port(v: String) -> Result<(), String> {
    let port_res = v.parse::<u16>();
    let _ = match port_res {
        Ok(_) => return Ok(()),
        Err(_) => return Err(String::from("port  must between 0~65535")),
    };
}

pub fn vali_proxy(v: String) -> Result<(), String> {
    let re_proxy  = Regex::new(r"((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})(\.((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})){3}").unwrap();
    let ans = re_proxy.is_match(v.as_str());
    if ans {
        Ok(())
    }else{
        Err(String::from("Invalid proxy input:\nhttp://127.0.0.1:8080, socks4:\\\\... socks5:\\\\... https:\\\\..."))
    }
}

pub fn vali_wt(v: String) -> Result<(), String> {
    let wt_res = v.parse::<f32>();
    let _ = match wt_res {
        Ok(wt) => {
            match wt {
                wt if wt <= 0.0 || wt >10.0 => return Err(String::from("wait time is between 0~10 S")),
                _ =>return Ok(()),
            }
        },
        Err(_) => return Err(String::from("port  must between 0~65535")),
    };
        
}



