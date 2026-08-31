<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index";
</script>

<Sidebar.Root>
    <Sidebar.Header>
        <span>Investment Tracker</span>
    </Sidebar.Header>
    <Sidebar.Content>
        <Sidebar.Menu>
            <Sidebar.MenuItem>
                <Sidebar.MenuButton>
                    <a href="/">Dashboard</a>
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>

            <Sidebar.MenuItem>
                <Sidebar.MenuButton>
                    <a href="/accounts">Accounts</a>
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>

            <Sidebar.MenuItem>
                <Sidebar.MenuButton>
                    <a href="/">Assets</a>
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>

        </Sidebar.Menu>
    </Sidebar.Content>

    <Sidebar.Footer>
        <Sidebar.Menu>
            <Sidebar.MenuItem>
                <Sidebar.MenuButton>
                    <a href="/profile">Profile</a>
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        </Sidebar.Menu>
    </Sidebar.Footer>
</Sidebar.Root>