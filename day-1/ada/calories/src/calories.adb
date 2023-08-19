with Ada.Text_IO;  use Ada.Text_IO;

procedure Calories is
   Input_File : File_Type;
   Highest_1 : Integer := 0;
   Highest_2 : Integer := 0;
   Highest_3 : Integer := 0;
   Sum : Integer := 0;
   Total : Integer := 0;

begin

   Open (File => Input_File, Mode => In_File, Name => "inputs.txt");
   while not End_Of_File (Input_File) loop
      declare
         Line : String := Get_Line (Input_File);
         Num : Integer := 0;
      begin
         if Line = "" then
            if Sum > Highest_1 then
               Highest_3 := Highest_2;
               Highest_2 := Highest_1;
               Highest_1 := Sum;
            elsif Sum > Highest_2 then
               Highest_3 := Highest_2;
               Highest_2 := Sum;
            elsif Sum > Highest_3 then
               Highest_3 := Sum;
            end if;
            Sum := 0;
         else
            Num := Integer'Value (Line);
            Sum := Sum + Num;
         end if;
      end;
   end loop;
   Close (File => Input_File);
   if Sum > Highest_1 then
      Highest_3 := Highest_2;
      Highest_2 := Highest_1;
      Highest_1 := Sum;
   elsif Sum > Highest_2 then
      Highest_3 := Highest_2;
      Highest_2 := Sum;
   elsif Sum > Highest_3 then
      Highest_3 := Sum;
   end if;
   Total := Highest_1 + Highest_2 + Highest_3;
   Put_Line ("Highest is: " & Highest_1'Img);
   Put_Line ("Total is: " & Total'Img);

end Calories;
