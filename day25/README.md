This document contains all the information about the maze that I encountered.
Which eventually lead to the main solution. The format is as follows:

Room Name (Item Name(s)):
 - Direction -> Goes To

### Mapping 

hull breach (none):
 - north -> corridor
 - east -> storage
 - west -> sick bay

corridor (infinite loop):
 - north -> stables
 - east -> passages
 - south -> hull breach

stables (mutex):
 - east -> engineering
 - south -> corridor
 - west -> kitchen

engineering (antenna):
 - west -> stables

kitchen (none):
 - east -> stables

passages (cake):
 - north -> gift wrapping center
 - east -> arcade
 - west -> corridor

gift wrapping center (none):
 - south -> passages

arcade (escape pod):
 - north -> navigation
 - west -> passages

navigation (pointer):
 - south -> arcade

storage (giant electromagnet)
 - east -> observatory
 - west -> hull breach

observatory (tambourine):
- north -> science lab
- east -> holodeck
- west -> storage

science lab (none):
 - south -> bservatory

holodeck (fuel cell):
 - east -> hallway
 - west -> observatory

hallway (boulder):
- north -> security checkpoint
- west -> holodeck

security checkpoint (none):
 - east -> goal
 - south -> hallway

sick bay (photons):
 - east -> Hull Breach
 - west -> hot chocolate fountain

hot chocolate fountain (none):
 - north -> warp drive maintenance
 - east -> sick bay
 - west -> crew quarters

warp drive maintenance (molten lava):
 - south -> hot chocolate fountain

crew quarters (coin):
 - east -> hot chocolate fountain
