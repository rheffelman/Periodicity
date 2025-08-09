# 1. Finish Stats Menu - Ryan
1. Each level the player should be given 5 points that they can spend towards whichever stat they choose. So at level 5 they should have 25 stat points total to spend, after they have acquired 5 points at level 1, 5 points at level 2, 5 points at level 3, and so on.
2. The dark area at the top of the menu should show how many points total the player has, how many they have spent, and how many they can spend.
3. Each stat area should have a button that when pressed, allocates 1 point towards the given stat, if the player has any points left to allocate. 

    You can see `create_side_buttons()` in `construct_window.rs` to see how I create several similar buttons.

    You can see `branch_from_click` in `button_definitions.rs` to see how to define what happens when buttons are pressed.

    To see how stats are stored, you can see `g_properties.rs` and `g_entities.rs`.

# 2. Finish Level Up System - Ryan
1. When the player kills an enemy, they should get xp. When the player gains xp, the purple xp bar at the bottom left should update accordingly.


# 3. Fix Scaling - Ryan
1. Currently, things don't look right when scaled below 1920x1080. Not sure why.

# 4. Fix Animations - Ryan
1. Currently, the animation that's supposed to happen when the player casts an ability just doesn't happen. Not sure why, I had it working before.

# 5. Finish Debuff Bar - Ryan
1. The debuff icons should start on the left side of the enemy info region under the healthbar, and grow right with each additional debuff, and also collapsing back left each time a debuff expires. currently it doesn't collapse back when icons dissappear. Also, there should be some text on the icon showing the remaining duration on the target. Debuff bar stuff mostly happens in `systems.rs`

# 6. Add + or - to Floating Combat Text - Ryan
1. When the player does say 5 damage to an enemy, it should say -5, currently it just says 5. If the player somehow heals the enemy, or the enemy heals itself, it should say +5. Floating combat text stuff is mostly done in `systems.rs`.