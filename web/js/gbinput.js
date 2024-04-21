export const GBInput = (() => {
    // in order of: START, SELECT, B, A, DOWN, UP, LEFT, RIGHT.
    const KEYMAPPINGS = [
        'Enter',
        'ShiftRight',
        'z',
        'x',
        'ArrowDown',
        'ArrowUp',
        'ArrowLeft',
        'ArrowRight',
    ];

    let keyStatus = 0xFF;

    window.addEventListener('keydown', (event) => {
        for (let i = 0; i < 8; i++) {
            if (event.key == KEYMAPPINGS[i]) {
                keyStatus &= ~(1 << (7 - i));
            }
        }
    });

    window.addEventListener('keyup', (event) => {
        for (let i = 0; i < 8; i++) {
            if (event.key == KEYMAPPINGS[i]) {
                keyStatus |= 1 << (7 - i)
            }
        }
    });

    return {
        getKeyStatus: () => {
            return keyStatus;
        },
    }
})();