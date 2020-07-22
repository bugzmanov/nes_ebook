# Implementing the rest of CPU instructions


 <div style="text-align:center"><img src="./images/ch3.3/image_1_how_to_draw_owl.png" width="60%"/></div>

Implementing the rest of 6502 CPU instructions should be relatively straightforward. I wouldn't go into details with every each one of them. 

I would just put some remarks:
* **ADC** is perhaps the most complicated instruction from logic flow perspective. Note that the spec contains details regarding decimal mode, that can be fully skipped, because Ricoh modification of the chip didn't support decimal mode, thus there is no games that use this functionallity.
> This article goes into detailed overview how binary arithmetic is implemented in 6502: [The 6502 overflow flag explained mathematically ](http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html)
>
>For the curious and brave souls: [The 6502 CPU's overflow flag explained at the silicon level ](http://www.righto.com/2013/01/a-small-part-of-6502-chip-explained.html)

* After ADC is implemented, implementing **SBC** becomes trivial as
`A - B = A + (-B)`. 
And `-B = !B + 1` 

* **PHP**, **PLP** and **RTI** have to deal with [2 bit B-flag](http://wiki.nesdev.com/w/index.php/Status_flags#The_B_flag). Those are the only commands that directly influence (or being directly influenced by) 5th Bit of **Status register P**

* Majority of branching and jumping operations can be implemented by simply modifying **program_counter** register. However be carefull not to modify the register afterwards within the same instruction interpret cycle. 

if you get stuck, you can always look up the implementaion of 6502 instruction set here: <link to code>


