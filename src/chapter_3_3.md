# Implementing the rest of CPU instructions


 <div style="text-align:center"><img src="./images/ch3.3/image_1_how_to_draw_owl.png" width="60%"/></div>

Implementing the rest of the 6502 CPU instructions should be relatively straightforward. I wouldn't go into detail with every one of them. 

I would just put some remarks:
* **ADC** is perhaps the most complicated instruction from a logic flow perspective. Note that the spec contains details regarding decimal mode, that can be entirely skipped because Ricoh modification of the chip didn't support decimal mode.
> This article goes into a detailed overview of how binary arithmetic is implemented in 6502: [The 6502 overflow flag explained mathematically ](http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html)
>
>For the curious and brave souls: [The 6502 CPU's overflow flag explained at the silicon level ](http://www.righto.com/2013/01/a-small-part-of-6502-chip-explained.html)

* After ADC is implemented, implementing **SBC** becomes trivial as
`A - B = A + (-B)`. 
And `-B = !B + 1` 

* **PHP**, **PLP** and **RTI** have to deal with [2 bit B-flag](http://wiki.nesdev.com/w/index.php/Status_flags#The_B_flag). Those are the only commands that directly influence (or being directly influenced by) 5th Bit of **Status register P**

* Majority of branching and jumping operations can be implemented by merely modifying **program_counter** register. However, be careful not to modify the register afterward within the same instruction interpret cycle. 

If you get stuck, you can always look up the implementation of 6502 instruction set here: <link to code>


