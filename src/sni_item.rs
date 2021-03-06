// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

#![allow(dead_code)]
use dbus as dbus;
use dbus::arg;
use dbus::tree;

pub trait OrgKdeStatusNotifierItem {
    type Err;
    fn scroll(&self, delta: i32, orientation: &str) -> Result<(), Self::Err>;
    fn activate(&self, x: i32, y: i32) -> Result<(), Self::Err>;
    fn secondary_activate(&self, x: i32, y: i32) -> Result<(), Self::Err>;
    fn get_id(&self) -> Result<String, Self::Err>;
    fn get_category(&self) -> Result<String, Self::Err>;
    fn get_status(&self) -> Result<String, Self::Err>;
    fn get_icon_name(&self) -> Result<String, Self::Err>;
    fn get_icon_pix_map(&self) -> Result<Vec<(i32, i32, Vec<u8>)>, Self::Err>;
    fn get_attention_icon_name(&self) -> Result<String, Self::Err>;
    fn get_title(&self) -> Result<String, Self::Err>;
    fn get_icon_theme_path(&self) -> Result<String, Self::Err>;
    fn get_menu(&self) -> Result<dbus::Path<'static>, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgKdeStatusNotifierItem for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn scroll(&self, delta: i32, orientation: &str) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.kde.StatusNotifierItem".into(), &"Scroll".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(delta);
            i.append(orientation);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn activate(&self, x: i32, y: i32) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.kde.StatusNotifierItem".into(), &"Activate".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(x);
            i.append(y);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn secondary_activate(&self, x: i32, y: i32) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.kde.StatusNotifierItem".into(), &"SecondaryActivate".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(x);
            i.append(y);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn get_id(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.kde.StatusNotifierItem", "Id")
    }

    fn get_category(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.kde.StatusNotifierItem", "Category")
    }

    fn get_status(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.kde.StatusNotifierItem", "Status")
    }

    fn get_icon_name(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.kde.StatusNotifierItem", "IconName")
    }

    fn get_icon_pix_map(&self) -> Result<Vec<(i32, i32, Vec<u8>)>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.kde.StatusNotifierItem", "IconPixmap")
    }

    fn get_attention_icon_name(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.kde.StatusNotifierItem", "AttentionIconName")
    }

    fn get_title(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.kde.StatusNotifierItem", "Title")
    }

    fn get_icon_theme_path(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.kde.StatusNotifierItem", "IconThemePath")
    }

    fn get_menu(&self) -> Result<dbus::Path<'static>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.kde.StatusNotifierItem", "Menu")
    }
}

pub fn org_kde_status_notifier_item_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    T: OrgKdeStatusNotifierItem<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.kde.StatusNotifierItem", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let delta: i32 = try!(i.read());
        let orientation: &str = try!(i.read());
        let d = fclone(minfo);
        try!(d.scroll(delta, orientation));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Scroll", Default::default(), h);
    let m = m.in_arg(("delta", "i"));
    let m = m.in_arg(("orientation", "s"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let x: i32 = try!(i.read());
        let y: i32 = try!(i.read());
        let d = fclone(minfo);
        try!(d.activate(x, y));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Activate", Default::default(), h);
    let m = m.in_arg(("x", "i"));
    let m = m.in_arg(("y", "i"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let x: i32 = try!(i.read());
        let y: i32 = try!(i.read());
        let d = fclone(minfo);
        try!(d.secondary_activate(x, y));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("SecondaryActivate", Default::default(), h);
    let m = m.in_arg(("x", "i"));
    let m = m.in_arg(("y", "i"));
    let i = i.add_m(m);

    let p = factory.property::<&str, _>("Id", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_id()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Category", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_category()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Status", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_status()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("IconName", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_icon_name()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<Vec<(i32, i32, Vec<u8>)>, _>("IconPixmap", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_icon_pix_map()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("AttentionIconName", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_attention_icon_name()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Title", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_title()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("IconThemePath", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_icon_theme_path()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<dbus::Path, _>("Menu", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_menu()));
        Ok(())
    });
    let i = i.add_p(p);
    i
}

#[derive(Debug, Default)]
pub struct OrgKdeStatusNotifierItemNewIcon {
}

impl dbus::SignalArgs for OrgKdeStatusNotifierItemNewIcon {
    const NAME: &'static str = "NewIcon";
    const INTERFACE: &'static str = "org.kde.StatusNotifierItem";
    fn append(&self, _: &mut arg::IterAppend) {
    }
    fn get(&mut self, _: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OrgKdeStatusNotifierItemNewIconThemePath {
    pub icon_theme_path: String,
}

impl dbus::SignalArgs for OrgKdeStatusNotifierItemNewIconThemePath {
    const NAME: &'static str = "NewIconThemePath";
    const INTERFACE: &'static str = "org.kde.StatusNotifierItem";
    fn append(&self, i: &mut arg::IterAppend) {
        (&self.icon_theme_path as &arg::RefArg).append(i);
    }
    fn get(&mut self, i: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        self.icon_theme_path = try!(i.read());
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OrgKdeStatusNotifierItemNewAttentionIcon {
}

impl dbus::SignalArgs for OrgKdeStatusNotifierItemNewAttentionIcon {
    const NAME: &'static str = "NewAttentionIcon";
    const INTERFACE: &'static str = "org.kde.StatusNotifierItem";
    fn append(&self, _: &mut arg::IterAppend) {
    }
    fn get(&mut self, _: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OrgKdeStatusNotifierItemNewStatus {
    pub status: String,
}

impl dbus::SignalArgs for OrgKdeStatusNotifierItemNewStatus {
    const NAME: &'static str = "NewStatus";
    const INTERFACE: &'static str = "org.kde.StatusNotifierItem";
    fn append(&self, i: &mut arg::IterAppend) {
        (&self.status as &arg::RefArg).append(i);
    }
    fn get(&mut self, i: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        self.status = try!(i.read());
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OrgKdeStatusNotifierItemNewTitle {
}

impl dbus::SignalArgs for OrgKdeStatusNotifierItemNewTitle {
    const NAME: &'static str = "NewTitle";
    const INTERFACE: &'static str = "org.kde.StatusNotifierItem";
    fn append(&self, _: &mut arg::IterAppend) {
    }
    fn get(&mut self, _: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        Ok(())
    }
}
