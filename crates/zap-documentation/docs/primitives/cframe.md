Zap supports sending CFrames. There are two types of CFrame you may send - a regular `CFrame`, and an `AlignedCFrame`.

CFrame rotation matrices are compressed using the axis-angle representation.

# Serialization Behavior

CFrames are orthonormalized when sent. If you need to send a CFrame that is not orthogonal, meaning one that does not have a valid rotation matrix, it is recommended to send the components and reconstruct it on the other side.
Note that use cases for this are exceedingly rare and you most likely will not have to worry about this, as the common CFrame constructors only return orthogonal CFrames.

### Aligned CFrames

When you know that a CFrame is going to be axis-aligned, it is preferrable to use the `AlignedCFrame` type.

It uses much less bandwidth, as the rotation can just be represented as a single byte Enum of the possible axis aligned rotations.

You can think of an axis-aligned CFrame as one whose LookVector, UpVector, and RightVector all align with the world axes in some way. Even if the RightVector is facing upwards, for example, it would still be axis-aligned.

Position does not matter at all, only the rotation.

# Warning

If the CFrame is not axis aligned then Zap will throw an error, so make sure to use this type carefully! Don't let this dissuade you from using it though, as the bandwidth savings can be significant.

Here are some examples of axis-aligned CFrames.

```luau
local CFrameSpecialCases = {
	CFrame.Angles(0, 0, 0),
	CFrame.Angles(0, math.rad(180), math.rad(90)),
	CFrame.Angles(0, math.rad(-90), 0),
	CFrame.Angles(math.rad(90), math.rad(-90), 0),
	CFrame.Angles(0, math.rad(90), math.rad(180)),
	-- and so on. there are 24 of these in total.
}
```
