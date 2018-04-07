//Code automatically generated by SOLSTICE
pragma solidity ^0.4.19;
contract my_bux {
  //Permissions and empowerments.
  //Exogenous events.
  bool burn_perm_wildcard_on_0 = false;
  bool burn_pow_wildcard_on_0 = false;
  mapping (address => bool) private burn_perm_map_on_0;
  mapping (address => bool) private burn_pow_map_on_0;
  bool burn_perm_wildcard_on_1 = false;
  bool burn_pow_wildcard_on_1 = false;
  mapping (address => bool) private burn_perm_map_on_1;
  mapping (address => bool) private burn_pow_map_on_1;
  bool create_contract_perm_wildcard_on_0 = true;
  bool create_contract_pow_wildcard_on_0 = true;
  mapping (address => bool) private create_contract_perm_map_on_0;
  mapping (address => bool) private create_contract_pow_map_on_0;
  bool freeze_perm_wildcard_on_0 = false;
  bool freeze_pow_wildcard_on_0 = false;
  mapping (address => bool) private freeze_perm_map_on_0;
  mapping (address => bool) private freeze_pow_map_on_0;
  bool freeze_perm_wildcard_on_1 = false;
  bool freeze_pow_wildcard_on_1 = false;
  mapping (address => bool) private freeze_perm_map_on_1;
  mapping (address => bool) private freeze_pow_map_on_1;
  bool mint_perm_wildcard_on_0 = false;
  bool mint_pow_wildcard_on_0 = false;
  mapping (address => bool) private mint_perm_map_on_0;
  mapping (address => bool) private mint_pow_map_on_0;
  bool mint_perm_wildcard_on_1 = false;
  bool mint_pow_wildcard_on_1 = false;
  mapping (address => bool) private mint_perm_map_on_1;
  mapping (address => bool) private mint_pow_map_on_1;
  bool permit_perm_wildcard_on_0 = false;
  bool permit_pow_wildcard_on_0 = false;
  mapping (address => bool) private permit_perm_map_on_0;
  mapping (address => bool) private permit_pow_map_on_0;
  bool permit_perm_wildcard_on_1 = false;
  bool permit_pow_wildcard_on_1 = false;
  mapping (address => bool) private permit_perm_map_on_1;
  mapping (address => bool) private permit_pow_map_on_1;
  bool transfer_perm_wildcard_on_0 = false;
  bool transfer_pow_wildcard_on_0 = false;
  mapping (address => bool) private transfer_perm_map_on_0;
  mapping (address => bool) private transfer_pow_map_on_0;
  bool transfer_perm_wildcard_on_1 = false;
  bool transfer_pow_wildcard_on_1 = false;
  mapping (address => bool) private transfer_perm_map_on_1;
  mapping (address => bool) private transfer_pow_map_on_1;
  bool transfer_perm_wildcard_on_2 = false;
  bool transfer_pow_wildcard_on_2 = false;
  mapping (address => bool) private transfer_perm_map_on_2;
  mapping (address => bool) private transfer_pow_map_on_2;
  
  //Inst events.
  bool balance_down_perm_wildcard_on_0 = true;
  bool balance_down_pow_wildcard_on_0 = false;
  mapping (address => bool) private balance_down_perm_map_on_0;
  mapping (address => bool) private balance_down_pow_map_on_0;
  bool balance_up_perm_wildcard_on_0 = true;
  bool balance_up_pow_wildcard_on_0 = false;
  mapping (address => bool) private balance_up_perm_map_on_0;
  mapping (address => bool) private balance_up_pow_map_on_0;
  bool inst_burn_perm_wildcard_on_0 = false;
  bool inst_burn_pow_wildcard_on_0 = false;
  mapping (address => bool) private inst_burn_perm_map_on_0;
  mapping (address => bool) private inst_burn_pow_map_on_0;
  bool inst_burn_perm_wildcard_on_1 = false;
  bool inst_burn_pow_wildcard_on_1 = false;
  mapping (address => bool) private inst_burn_perm_map_on_1;
  mapping (address => bool) private inst_burn_pow_map_on_1;
  bool inst_creation_perm_wildcard_on_0 = true;
  bool inst_creation_pow_wildcard_on_0 = false;
  mapping (address => bool) private inst_creation_perm_map_on_0;
  mapping (address => bool) private inst_creation_pow_map_on_0;
  bool inst_freeze_perm_wildcard_on_0 = false;
  bool inst_freeze_pow_wildcard_on_0 = false;
  mapping (address => bool) private inst_freeze_perm_map_on_0;
  mapping (address => bool) private inst_freeze_pow_map_on_0;
  bool inst_freeze_perm_wildcard_on_1 = false;
  bool inst_freeze_pow_wildcard_on_1 = false;
  mapping (address => bool) private inst_freeze_perm_map_on_1;
  mapping (address => bool) private inst_freeze_pow_map_on_1;
  bool inst_mint_perm_wildcard_on_0 = false;
  bool inst_mint_pow_wildcard_on_0 = false;
  mapping (address => bool) private inst_mint_perm_map_on_0;
  mapping (address => bool) private inst_mint_pow_map_on_0;
  bool inst_mint_perm_wildcard_on_1 = false;
  bool inst_mint_pow_wildcard_on_1 = false;
  mapping (address => bool) private inst_mint_perm_map_on_1;
  mapping (address => bool) private inst_mint_pow_map_on_1;
  bool inst_permit_perm_wildcard_on_0 = false;
  bool inst_permit_pow_wildcard_on_0 = false;
  mapping (address => bool) private inst_permit_perm_map_on_0;
  mapping (address => bool) private inst_permit_pow_map_on_0;
  bool inst_permit_perm_wildcard_on_1 = false;
  bool inst_permit_pow_wildcard_on_1 = false;
  mapping (address => bool) private inst_permit_perm_map_on_1;
  mapping (address => bool) private inst_permit_pow_map_on_1;
  bool inst_transfer_perm_wildcard_on_0 = false;
  bool inst_transfer_pow_wildcard_on_0 = false;
  mapping (address => bool) private inst_transfer_perm_map_on_0;
  mapping (address => bool) private inst_transfer_pow_map_on_0;
  bool inst_transfer_perm_wildcard_on_1 = false;
  bool inst_transfer_pow_wildcard_on_1 = false;
  mapping (address => bool) private inst_transfer_perm_map_on_1;
  mapping (address => bool) private inst_transfer_pow_map_on_1;
  bool inst_transfer_perm_wildcard_on_2 = false;
  bool inst_transfer_pow_wildcard_on_2 = false;
  mapping (address => bool) private inst_transfer_perm_map_on_2;
  mapping (address => bool) private inst_transfer_pow_map_on_2;
  
  //Fluents
  mapping(address => bool) public frozen_0;
  mapping(address => bool) public has_money_0;
  mapping(address => bool) public is_owner_0;
  
  //Violation events
  event not_enough_money(address a);
  
  event not_owner(address a);
  
  function my_bux() public {
    //Function calls for initial 'create_contract' event.
    inst_creation(msg.sender);
  }
  function balance_down(address a) private {
    require((balance_down_perm_wildcard_on_0 || balance_down_perm_map_on_0[a]));
    has_money_0[a] = false;
  }
  function balance_up(address a) private {
    require((balance_up_perm_wildcard_on_0 || balance_up_perm_map_on_0[a]));
    has_money_0[a] = true;
  }
  function inst_burn(address a, address b) private {
    require((inst_burn_perm_wildcard_on_0 || inst_burn_perm_map_on_0[a]) && (inst_burn_perm_wildcard_on_1 || inst_burn_perm_map_on_1[b]));
    if(has_money_0[b]){
      balance_down(b);
    }
    if(!has_money_0[b]){
      not_enough_money(b);
    }
  }
  function inst_creation(address a) private {
    require((inst_creation_perm_wildcard_on_0 || inst_creation_perm_map_on_0[a]));
    permit_perm_wildcard_on_0 = true;
    permit_perm_wildcard_on_1 = true;
    permit_pow_wildcard_on_0 = true;
    permit_pow_wildcard_on_1 = true;
    inst_permit_perm_wildcard_on_0 = true;
    inst_permit_perm_wildcard_on_1 = true;
    freeze_perm_map_on_0[a] = true;
    freeze_perm_wildcard_on_1 = true;
    freeze_pow_map_on_0[a] = true;
    freeze_pow_wildcard_on_1 = true;
    inst_freeze_perm_map_on_0[a] = true;
    inst_freeze_perm_wildcard_on_1 = true;
    freeze_perm_wildcard_on_0 = true;
    freeze_perm_wildcard_on_1 = true;
    freeze_pow_wildcard_on_0 = true;
    freeze_pow_wildcard_on_1 = true;
    inst_freeze_perm_wildcard_on_0 = true;
    inst_freeze_perm_wildcard_on_1 = true;
    burn_perm_map_on_0[a] = true;
    burn_perm_wildcard_on_1 = true;
    burn_pow_map_on_0[a] = true;
    burn_pow_wildcard_on_1 = true;
    inst_burn_perm_map_on_0[a] = true;
    inst_burn_perm_wildcard_on_1 = true;
    burn_perm_wildcard_on_0 = true;
    burn_perm_wildcard_on_1 = true;
    burn_pow_wildcard_on_0 = true;
    burn_pow_wildcard_on_1 = true;
    inst_burn_perm_wildcard_on_0 = true;
    inst_burn_perm_wildcard_on_1 = true;
    mint_perm_map_on_0[a] = true;
    mint_perm_wildcard_on_1 = true;
    mint_pow_map_on_0[a] = true;
    mint_pow_wildcard_on_1 = true;
    inst_mint_perm_map_on_0[a] = true;
    inst_mint_perm_wildcard_on_1 = true;
    transfer_perm_wildcard_on_0 = true;
    transfer_perm_wildcard_on_1 = true;
    transfer_perm_wildcard_on_2 = true;
    transfer_pow_wildcard_on_0 = true;
    transfer_pow_wildcard_on_1 = true;
    transfer_pow_wildcard_on_2 = true;
    inst_transfer_perm_wildcard_on_0 = true;
    inst_transfer_perm_wildcard_on_1 = true;
    inst_transfer_perm_wildcard_on_2 = true;
    is_owner_0[a] = true;
    balance_up(a);
    create_contract_perm_wildcard_on_0 = false;
    create_contract_pow_wildcard_on_0 = false;
  }
  function inst_freeze(address a, address b) private {
    require((inst_freeze_perm_wildcard_on_0 || inst_freeze_perm_map_on_0[a]) && (inst_freeze_perm_wildcard_on_1 || inst_freeze_perm_map_on_1[b]));
    transfer_perm_wildcard_on_0 = false;
    transfer_perm_wildcard_on_1 = false;
    transfer_perm_map_on_2[b] = false;
    transfer_pow_wildcard_on_0 = false;
    transfer_pow_wildcard_on_1 = false;
    transfer_pow_map_on_2[b] = false;
    inst_transfer_perm_wildcard_on_0 = false;
    inst_transfer_perm_wildcard_on_1 = false;
    inst_transfer_perm_map_on_2[b] = false;
    transfer_perm_wildcard_on_0 = false;
    transfer_perm_map_on_1[b] = false;
    transfer_perm_wildcard_on_2 = false;
    transfer_pow_wildcard_on_0 = false;
    transfer_pow_map_on_1[b] = false;
    transfer_pow_wildcard_on_2 = false;
    inst_transfer_perm_wildcard_on_0 = false;
    inst_transfer_perm_map_on_1[b] = false;
    inst_transfer_perm_wildcard_on_2 = false;
  }
  function inst_mint(address a, address b) private {
    require((inst_mint_perm_wildcard_on_0 || inst_mint_perm_map_on_0[a]) && (inst_mint_perm_wildcard_on_1 || inst_mint_perm_map_on_1[b]));
    balance_up(b);
  }
  function inst_permit(address a, address b) private {
    require((inst_permit_perm_wildcard_on_0 || inst_permit_perm_map_on_0[a]) && (inst_permit_perm_wildcard_on_1 || inst_permit_perm_map_on_1[b]));
    burn_perm_map_on_0[b] = true;
    burn_perm_map_on_1[a] = true;
    burn_pow_map_on_0[b] = true;
    burn_pow_map_on_1[a] = true;
    inst_burn_perm_map_on_0[b] = true;
    inst_burn_perm_map_on_1[a] = true;
    transfer_perm_map_on_0[b] = true;
    transfer_perm_map_on_1[a] = true;
    transfer_perm_wildcard_on_2 = true;
    transfer_pow_map_on_0[b] = true;
    transfer_pow_map_on_1[a] = true;
    transfer_pow_wildcard_on_2 = true;
    inst_transfer_perm_map_on_0[b] = true;
    inst_transfer_perm_map_on_1[a] = true;
    inst_transfer_perm_wildcard_on_2 = true;
  }
  function inst_transfer(address a, address b, address c) private {
    require((inst_transfer_perm_wildcard_on_0 || inst_transfer_perm_map_on_0[a]) && (inst_transfer_perm_wildcard_on_1 || inst_transfer_perm_map_on_1[b]) && (inst_transfer_perm_wildcard_on_2 || inst_transfer_perm_map_on_2[c]));
    if(has_money_0[b]){
      balance_down(b);
      balance_up(c);
    }
    if(!has_money_0[b]){
      not_enough_money(b);
    }
  }
  function burn(address b) public {
    address a = msg.sender;
    require(((burn_perm_wildcard_on_0 || burn_perm_map_on_0[a]) && (burn_pow_wildcard_on_0 || burn_pow_map_on_0[a])) && ((burn_perm_wildcard_on_1 || burn_perm_map_on_1[b]) && (burn_pow_wildcard_on_1 || burn_pow_map_on_1[b])));
    inst_burn(a,b);
  }
  function create_contract() public {
    address a = msg.sender;
    require(((create_contract_perm_wildcard_on_0 || create_contract_perm_map_on_0[a]) && (create_contract_pow_wildcard_on_0 || create_contract_pow_map_on_0[a])));
    inst_creation(a);
  }
  function freeze(address b) public {
    address a = msg.sender;
    require(((freeze_perm_wildcard_on_0 || freeze_perm_map_on_0[a]) && (freeze_pow_wildcard_on_0 || freeze_pow_map_on_0[a])) && ((freeze_perm_wildcard_on_1 || freeze_perm_map_on_1[b]) && (freeze_pow_wildcard_on_1 || freeze_pow_map_on_1[b])));
    inst_freeze(a,b);
  }
  function mint(address b) public {
    address a = msg.sender;
    require(((mint_perm_wildcard_on_0 || mint_perm_map_on_0[a]) && (mint_pow_wildcard_on_0 || mint_pow_map_on_0[a])) && ((mint_perm_wildcard_on_1 || mint_perm_map_on_1[b]) && (mint_pow_wildcard_on_1 || mint_pow_map_on_1[b])));
    inst_mint(a,b);
  }
  function permit(address b) public {
    address a = msg.sender;
    require(((permit_perm_wildcard_on_0 || permit_perm_map_on_0[a]) && (permit_pow_wildcard_on_0 || permit_pow_map_on_0[a])) && ((permit_perm_wildcard_on_1 || permit_perm_map_on_1[b]) && (permit_pow_wildcard_on_1 || permit_pow_map_on_1[b])));
    inst_permit(a,b);
  }
  function transfer(address b, address c) public {
    address a = msg.sender;
    require(((transfer_perm_wildcard_on_0 || transfer_perm_map_on_0[a]) && (transfer_pow_wildcard_on_0 || transfer_pow_map_on_0[a])) && ((transfer_perm_wildcard_on_1 || transfer_perm_map_on_1[b]) && (transfer_pow_wildcard_on_1 || transfer_pow_map_on_1[b])) && ((transfer_perm_wildcard_on_2 || transfer_perm_map_on_2[c]) && (transfer_pow_wildcard_on_2 || transfer_pow_map_on_2[c])));
    inst_transfer(a,b,c);
  }
}
