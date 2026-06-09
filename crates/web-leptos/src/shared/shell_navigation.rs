#[derive(Clone, PartialEq, Eq)]
pub struct NavItem {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: &'static str,
}

pub fn main_nav_items(
    include_profile: bool,
    is_admin: bool,
    include_trading: bool,
    include_portfolio: bool,
) -> Vec<NavItem> {
    let mut items = vec![NavItem {
        label: "Dashboard",
        href: "/",
        icon: "📊",
    }];
    if include_portfolio {
        items.push(NavItem {
            label: "Carteira",
            href: "/portfolio",
            icon: "💼",
        });
    }
    if include_trading {
        items.push(NavItem {
            label: "Operações",
            href: "/trades",
            icon: "🔄",
        });
    }
    if is_admin {
        items.push(NavItem {
            label: "Ativos",
            href: "/admin/assets",
            icon: "🏷️",
        });
    }
    if include_profile {
        items.push(NavItem {
            label: "Perfil",
            href: "/profile",
            icon: "👤",
        });
    }
    items
}
