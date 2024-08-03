<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" tiledversion="1.10.1" name="shape" tilewidth="16" tileheight="16" tilecount="1024" columns="32" tilerendersize="grid">
 <image source="shape.png" width="512" height="512"/>
 <tile id="66">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="floor-nw"/>
  </properties>
 </tile>
 <tile id="67">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="floor"/>
  </properties>
 </tile>
 <tile id="68">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="floor-ne"/>
  </properties>
 </tile>
 <tile id="72">
  <properties>
   <property name="cliff" value="new"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="floor"/>
  </properties>
  <objectgroup draworder="index" id="3">
   <object id="2" x="6.28437" y="4.27337">
    <point/>
   </object>
   <object id="3" x="8.37916" y="12.3174">
    <ellipse/>
   </object>
  </objectgroup>
 </tile>
 <tile id="76">
  <properties>
   <property name="cliff" value="ne"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="floor-nw"/>
  </properties>
 </tile>
 <tile id="78">
  <properties>
   <property name="cliff" value="nw"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="floor-ne"/>
  </properties>
 </tile>
 <tile id="98">
  <properties>
   <property name="cliff" value="w"/>
   <property name="shape" value="floor"/>
  </properties>
 </tile>
 <tile id="100">
  <properties>
   <property name="cliff" value="e"/>
   <property name="shape" value="floor"/>
  </properties>
 </tile>
 <tile id="103">
  <properties>
   <property name="cliff" value="nw"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="floor"/>
  </properties>
 </tile>
 <tile id="105">
  <properties>
   <property name="cliff" value="ne"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="floor"/>
  </properties>
 </tile>
 <tile id="109">
  <properties>
   <property name="cliff" value="ew"/>
   <property name="shape" value="floor"/>
  </properties>
 </tile>
 <tile id="130">
  <properties>
   <property name="shape" value="wall-floor-sw"/>
  </properties>
 </tile>
 <tile id="131">
  <properties>
   <property name="shape" value="floor"/>
  </properties>
 </tile>
 <tile id="132">
  <properties>
   <property name="shape" value="wall-floor-se"/>
  </properties>
 </tile>
 <tile id="135">
  <properties>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall-ne"/>
  </properties>
 </tile>
 <tile id="137">
  <properties>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall-nw"/>
  </properties>
 </tile>
 <tile id="140">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall-floor-sw"/>
  </properties>
 </tile>
 <tile id="142">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall-floor-se"/>
  </properties>
 </tile>
 <tile id="145">
  <properties>
   <property name="cliff" value="e"/>
   <property name="shape" value="wall-floor-sw"/>
  </properties>
 </tile>
 <tile id="147">
  <properties>
   <property name="cliff" value="w"/>
   <property name="shape" value="wall-floor-se"/>
  </properties>
 </tile>
 <tile id="150">
  <properties>
   <property name="cliff" value="n,e"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall-floor-sw"/>
  </properties>
 </tile>
 <tile id="152">
  <properties>
   <property name="cliff" value="n,w"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall-floor-se"/>
  </properties>
 </tile>
 <tile id="162">
  <properties>
   <property name="shape" value="floor-wall-sw"/>
  </properties>
 </tile>
 <tile id="163">
  <properties>
   <property name="shape" value="wall"/>
  </properties>
 </tile>
 <tile id="164">
  <properties>
   <property name="shape" value="floor-wall-se"/>
  </properties>
 </tile>
 <tile id="168">
  <properties>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall"/>
  </properties>
 </tile>
 <tile id="258">
  <properties>
   <property name="shape" value="slope-floor-sw"/>
  </properties>
 </tile>
 <tile id="260">
  <properties>
   <property name="shape" value="slope-floor-se"/>
  </properties>
 </tile>
 <tile id="263">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope-nw"/>
  </properties>
 </tile>
 <tile id="264">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="265">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope-ne"/>
  </properties>
 </tile>
 <tile id="268">
  <properties>
   <property name="cliff" value="ne"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope-nw"/>
  </properties>
 </tile>
 <tile id="270">
  <properties>
   <property name="cliff" value="nw"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope-ne"/>
  </properties>
 </tile>
 <tile id="274">
  <properties>
   <property name="cliff" value="w"/>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="279">
  <properties>
   <property name="cliff" value="n,w"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="290">
  <properties>
   <property name="shape" value="floor-slope-sw"/>
  </properties>
 </tile>
 <tile id="291">
  <properties>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="292">
  <properties>
   <property name="shape" value="floor-slope-se"/>
  </properties>
 </tile>
 <tile id="296">
  <properties>
   <property name="cliff" value="ew"/>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="301">
  <properties>
   <property name="cliff" value="new"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="306">
  <properties>
   <property name="cliff" value="e"/>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="311">
  <properties>
   <property name="cliff" value="n,e"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="333">
  <properties>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="338">
  <properties>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="343">
  <properties>
   <property name="shape" value="slope"/>
  </properties>
 </tile>
 <tile id="386">
  <properties>
   <property name="shape" value="wall-slope-sw"/>
  </properties>
 </tile>
 <tile id="387">
  <properties>
   <property name="shape" value="slope-wall-se"/>
  </properties>
 </tile>
 <tile id="388">
  <properties>
   <property name="shape" value="wall-slope-se"/>
  </properties>
 </tile>
 <tile id="391">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall-slope-sw"/>
  </properties>
 </tile>
 <tile id="393">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall-slope-se"/>
  </properties>
 </tile>
 <tile id="396">
  <properties>
   <property name="cliff" value="e"/>
   <property name="shape" value="wall-slope-sw"/>
  </properties>
 </tile>
 <tile id="398">
  <properties>
   <property name="cliff" value="w"/>
   <property name="shape" value="wall-slope-se"/>
  </properties>
 </tile>
 <tile id="401">
  <properties>
   <property name="cliff" value="ne"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall-slope-sw"/>
  </properties>
 </tile>
 <tile id="403">
  <properties>
   <property name="cliff" value="nw"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="wall-slope-se"/>
  </properties>
 </tile>
 <tile id="418">
  <properties>
   <property name="shape" value="slope-wall-sw"/>
  </properties>
 </tile>
 <tile id="419">
  <properties>
   <property name="shape" value="slope-wall-se"/>
  </properties>
 </tile>
 <tile id="420">
  <properties>
   <property name="shape" value="slope-wall-se"/>
  </properties>
 </tile>
 <tile id="423">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope-wall-sw"/>
  </properties>
 </tile>
 <tile id="425">
  <properties>
   <property name="cliff" value="n"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope-wall-se"/>
  </properties>
 </tile>
 <tile id="428">
  <properties>
   <property name="cliff" value="w"/>
   <property name="shape" value="slope-wall-sw"/>
  </properties>
 </tile>
 <tile id="430">
  <properties>
   <property name="cliff" value="e"/>
   <property name="shape" value="slope-wall-se"/>
  </properties>
 </tile>
 <tile id="433">
  <properties>
   <property name="cliff" value="nw"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope-wall-sw"/>
  </properties>
 </tile>
 <tile id="435">
  <properties>
   <property name="cliff" value="ne"/>
   <property name="reset" type="bool" value="true"/>
   <property name="shape" value="slope-wall-se"/>
  </properties>
 </tile>
</tileset>
