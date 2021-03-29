# orthogonal_rotation


rotates a point "orthogonally" around the origin


which I can't think of a good defintion for what I mean by "orthogonally"
but right now the best way to describe it is to show how it affects a set of points


An array of points
(the position in the array represents its cartesian coordinates)

 100  101  102  103  104  105  106  107  108 
 109  110  111  112  113  114  115  116  117 
 118  119  120  121  122  123  124  125  126 
 127  128  129  130  131  132  133  134  135 
 136  137  138  139  140  141  142  143  144 
 145  146  147  148  149  150  151  152  153 
 154  155  156  157  158  159  160  161  162 
 163  164  165  166  167  168  169  170  171 
 172  173  174  175  176  177  178  179  180 


rotated with value 140 representing the center, with the value 0.25
it rotates the array 90 degrees counterclockwise

 108  117  126  135  144  153  162  171  180 
 107  116  125  134  143  152  161  170  179 
 106  115  124  133  142  151  160  169  178 
 105  114  123  132  141  150  159  168  177 
 104  113  122  131  140  149  158  167  176 
 103  112  121  130  139  148  157  166  175 
 102  111  120  129  138  147  156  165  174 
 101  110  119  128  137  146  155  164  173 
 100  109  118  127  136  145  154  163  172 



Or rotated with 140 as the center with the value 0.125
it rotates the array so the diagonals become the x and y axis lines of the axis clockwise of them

 104  105  106  107  108  117  126  135  144 
 103  113  114  115  116  125  134  143  153 
 102  112  122  123  124  133  142  152  162 
 101  111  121  131  132  141  151  161  171 
 100  110  120  130  140  150  160  170  180 
 109  119  129  139  148  149  159  169  179 
 118  128  138  147  156  157  158  168  178 
 127  137  146  155  164  165  166  167  177 
 136  145  154  163  172  173  174  175  176 


Or at odder angles like 0.0287
which shifts each point 2 units clockwise while maintaining the same ( x + y instead of sqrt(x^2 + y^2) ) distance from the origin

 102  103  104  105  106  107  108  117  126 
 101  111  112  113  114  115  116  125  135 
 100  110  121  122  123  124  133  134  144 
 109  119  120  130  131  132  142  143  153 
 118  128  129  139  140  141  151  152  162 
 127  137  138  148  149  150  160  161  171 
 136  146  147  156  157  158  159  170  180 
 145  155  164  165  166  167  168  169  179 
 154  163  172  173  174  175  176  177  178



I made this for my chess game so that each of the 64 squares can be rotated an amount
and each square if rotated the same amount, will be mapped to a square that none of the other squares are mapped to


and it does some things in a way that are useful for my game