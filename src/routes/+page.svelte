<script lang="ts">
import { HSplitPane, VSplitPane } from 'svelte-split-pane';
</script>
<div class="h-full">
    <HSplitPane leftPaneSize="75%" rightPaneSize="25%" minLeftPaneSize="50px" minRightPaneSize="50px">
        <left slot="left">
            <VSplitPane topPanelSize="75%" downPanelSize="25%" minTopPaneSize="50px" minDownPaneSize="50px" >
                <top slot="top">
                    Top Pane
                </top>
                <down slot="down">
                    Down Pane
                </down>
            </VSplitPane>
        </left>
        <right slot="right">
            Right Pane
        </right>
    </HSplitPane>
</div>