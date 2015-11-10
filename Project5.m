%Euler Project5

i = 0;
x = 0;
a = 0;

x = factorial(20)/(4*6*8*15*18*16);

for i = 1:x
    
        if mod(i,2)== 0& mod(i,3)== 0&mod(i,4)== 0& mod(i,5)== 0& mod(i,6)== 0& mod(i,7)== 0 & mod(i,8)== 0&mod(i,9)== 0& mod(i,10)== 0&mod(i,11)== 0& mod(i,12)== 0&mod(i,13)== 0& mod(i,14)== 0& mod(i,15)== 0& mod(i,16)== 0 & mod(i,17)== 0&mod(i,18)== 0& mod(i,19)== 0&mod(i,20)==0
           a = i;
           break;
        end
    
end
a