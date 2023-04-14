<script lang="ts">
    import { Accordion, AccordionItem } from "@skeletonlabs/skeleton";
    import { UserIcon, ArchiveIcon, ToolIcon, TypeIcon } from "$lib/icons";
    import { LightSwitch } from "@skeletonlabs/skeleton";
    import { themeStore } from "$lib/theme.store";

    let activeSection = 0;
</script>

<div>
    <Accordion autocollapse>
        <AccordionItem open>
            <svelte:fragment slot="lead">
                <ToolIcon />
            </svelte:fragment>
            <svelte:fragment slot="summary">General</svelte:fragment>
            <svelte:fragment slot="content">
                <div class="flex flex-row gap-2">
                    <LightSwitch
                        bgDark="bg-surface-700"
                        on:click={() => themeStore.flip()}
                    />
                    <div class="label">
                        {$themeStore ? "Light" : "Dark"} theme
                    </div>
                </div>
            </svelte:fragment>
        </AccordionItem>
        <hr />
        <AccordionItem>
            <svelte:fragment slot="lead">
                <TypeIcon />
            </svelte:fragment>
            <svelte:fragment slot="summary">Character Sets</svelte:fragment>
            <svelte:fragment slot="content">
                <div class="p-5 backdrop-brightness-50">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Ut enim ad minim veniam, quis nostrud exercitation
                    ullamco laboris nisi ut aliquip ex ea commodo consequat.
                    Duis aute irure dolor in reprehenderit in voluptate velit
                    esse cillum dolore eu fugiat nulla pariatur. Excepteur sint
                    occaecat cupidatat non proident, sunt in culpa qui officia
                    deserunt mollit anim id est laborum.
                </div>
            </svelte:fragment>
        </AccordionItem>
        <hr />
        <AccordionItem>
            <svelte:fragment slot="lead">
                <UserIcon />
            </svelte:fragment>
            <svelte:fragment slot="summary">Profiles</svelte:fragment>
            <svelte:fragment slot="content">
                <div class="p-5 backdrop-brightness-50">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Ut enim ad minim veniam, quis nostrud exercitation
                    ullamco laboris nisi ut aliquip ex ea commodo consequat.
                    Duis aute irure dolor in reprehenderit in voluptate velit
                    esse cillum dolore eu fugiat nulla pariatur. Excepteur sint
                    occaecat cupidatat non proident, sunt in culpa qui officia
                    deserunt mollit anim id est laborum.
                </div>
            </svelte:fragment>
        </AccordionItem>
        <hr />
        <AccordionItem>
            <svelte:fragment slot="lead">
                <ArchiveIcon />
            </svelte:fragment>
            <svelte:fragment slot="summary">Data</svelte:fragment>
            <svelte:fragment slot="content">
                <div class="p-5 backdrop-brightness-50">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Ut enim ad minim veniam, quis nostrud exercitation
                    ullamco laboris nisi ut aliquip ex ea commodo consequat.
                    Duis aute irure dolor in reprehenderit in voluptate velit
                    esse cillum dolore eu fugiat nulla pariatur. Excepteur sint
                    occaecat cupidatat non proident, sunt in culpa qui officia
                    deserunt mollit anim id est laborum.
                </div>
            </svelte:fragment>
        </AccordionItem>
        <!-- ... -->
    </Accordion>
</div>
