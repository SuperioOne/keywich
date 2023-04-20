<script lang="ts">
    import { Paginator } from "@skeletonlabs/skeleton";

    const sourceData = [
        { position: 1, name: "Hydrogen", weight: 1.0079, symbol: "H" },
        { position: 2, name: "Helium", weight: 4.0026, symbol: "He" },
        { position: 3, name: "Lithium", weight: 6.941, symbol: "Li" },
        { position: 4, name: "Beryllium", weight: 9.0122, symbol: "Be" },
        { position: 5, name: "Boron", weight: 10.811, symbol: "B" },
    ];

    let page = {
        offset: 0,
        limit: 10,
        size: sourceData.length,
        amounts: [10, 25, 50, 100],
    };
</script>

<div class="table-container table-compact">
    <table class="table table-hover">
        <thead>
            <tr>
                <th>User Name</th>
                <th>Domain</th>
                <th>Access Time</th>
            </tr>
        </thead>
        <tbody>
            {#each sourceData as row, i}
                <tr>
                    <td>User Name</td>
                    <td>Domain</td>
                    <td>{new Date().toLocaleDateString()}</td>
                </tr>
            {/each}
        </tbody>
        <tfoot>
            <tr>
                <td colspan="3">
                    <Paginator
                    buttonClasses="btn-icon btn-icon-md variant-filled"
                    bind:settings={page}
                    />
                </td>
            </tr>
        </tfoot>
    </table>
</div>
