const POSITIONS = new Map<string, { display_name: string, description: string, color: string, level: number }>();

async function arrayPositionsToMap(array: Array<{ color: string, description: string, display_name: string, level: number, position: string }>) {
    POSITIONS.clear();
    for (const item of array) {
        POSITIONS.set(item.position, { display_name: item.display_name, description: item.description, color: item.color, level: item.level });
    }

    console.log(POSITIONS);
}

export { POSITIONS, arrayPositionsToMap };