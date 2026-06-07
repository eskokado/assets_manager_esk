#[derive(Clone, PartialEq, Eq)]
pub struct NavItem {
    pub label: &'static str,
    pub href: &'static str,
}

pub fn main_nav_items(
    include_profile: bool,
    is_admin: bool,
    include_trading: bool,
) -> Vec<NavItem> {
    let mut items = vec![
        NavItem {
            label: "Dashboard",
            href: "/",
        },
        NavItem {
            label: "Exemplos",
            href: "/examples",
        },
    ];
    if include_trading {
        items.push(NavItem {
            label: "Operações",
            href: "/trades",
        });
    }
    if is_admin {
        items.push(NavItem {
            label: "Ativos",
            href: "/admin/assets",
        });
    }
    if include_profile {
        items.push(NavItem {
            label: "Perfil",
            href: "/profile",
        });
    }
    items
}
