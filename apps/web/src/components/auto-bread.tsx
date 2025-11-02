"use client";
import React from "react";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "@workspace/ui/components/breadcrumb";
import { usePathname } from "next/navigation";

export default function AutoBread() {
    const pathname = usePathname();
    console.log("Current pathname:", pathname);

    const segments = pathname.split("/").filter(Boolean);
    return (
        <Breadcrumb className="py-1 px-5 mb-5 border-b border-foreground/40">
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink href="/">Home</BreadcrumbLink>
                </BreadcrumbItem>
                {segments.length > 0 && <BreadcrumbSeparator />}
                {segments.map((segment, index, array) => {
                    const href = "/" + array.slice(0, index + 1).join("/");
                    const isLast = index === array.length - 1;
                    return (
                        <React.Fragment key={href}>
                            <BreadcrumbItem>
                                {isLast ? (
                                    <BreadcrumbLink aria-current="page">{segment}</BreadcrumbLink>
                                ) : (
                                    <BreadcrumbLink href={href}>{segment}</BreadcrumbLink>
                                )}
                            </BreadcrumbItem>
                            {!isLast && <BreadcrumbSeparator />}
                        </React.Fragment>
                    );
                })}
            </BreadcrumbList>
        </Breadcrumb>
    )
}