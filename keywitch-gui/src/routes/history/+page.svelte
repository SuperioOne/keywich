<script lang="ts">
  const sourceData = [
    { position: 1, name: 'Hydrogen', weight: 1.0079, symbol: 'H' },
    { position: 2, name: 'Helium', weight: 4.0026, symbol: 'He' },
    { position: 3, name: 'Lithium', weight: 6.941, symbol: 'Li' },
    { position: 4, name: 'Beryllium', weight: 9.0122, symbol: 'Be' },
    { position: 5, name: 'Boron', weight: 10.811, symbol: 'B' },
  ];
</script>

<div class="table-container">
    <table class="table table-hover">
        <thead>
        <tr>
            <th>Position</th>
            <th>Name</th>
            <th>Symbol</th>
            <th>Weight</th>
        </tr>
        </thead>
        <tbody>
        {#each sourceData as row, i}
            <tr>
                <td>{row.position}</td>
                <td>{row.name}</td>
                <td>{row.symbol}</td>
                <td>{row.weight}</td>
            </tr>
        {/each}
        </tbody>
        <tfoot>
        <tr>
            <th colspan="3">Calculated Total Weight</th>
            <td>12345</td>
        </tr>
        </tfoot>
    </table>
</div>