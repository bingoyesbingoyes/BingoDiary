const routes = [
    {
        path: "/",
        component: () => import("pages/Index.vue"),
        // component: () => import("pages/Test.vue"),
    },
    // Always leave this as last one,
    // but you can also remove it
    {
        path: "/:catchAll(.*)*",
        component: () => import("pages/ErrorNotFound.vue"),
    },
];

export default routes;
