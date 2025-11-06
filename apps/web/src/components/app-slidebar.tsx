import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuItem } from "@workspace/ui/components/sidebar"
import Link from "next/link";

interface AppSidebarMenuListItemProps extends Record<"title" | "link", string> {}

export default function AppSlidebar() {

    const menuItems: AppSidebarMenuListItemProps[] = [
        { title: "思想", link: "/contents/thinks" },
        { title: "身体の計測データ", link: "/contents/body-metrics" },
        { title: "管理", link: "/manage" },
        { title: "服", link: "/manage/clothes" },
    ];

    return (
        <Sidebar>
            <SidebarHeader />
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>
                        コンテンツ一覧
                    </SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            {menuItems.map((item) => (
                                <SidebarMenuItem key={item.title}>
                                    <SidebarMenuButton asChild>
                                        <Link href={item.link}>
                                            {item.title}
                                        </Link>
                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                            ))}
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
                <SidebarGroup />
            </SidebarContent>
            <SidebarFooter />
        </Sidebar>
    )
}